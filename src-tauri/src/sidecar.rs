use std::path::PathBuf;
use std::process::{Child, Command};
use std::time::Duration;
use tokio::time::sleep;

pub struct SurrealDbSidecar {
    process: Option<Child>,
    data_path: PathBuf,
}

impl SurrealDbSidecar {
    /// Start SurrealDB sidecar process
    pub fn start(data_path: PathBuf) -> Result<Self, String> {
        tracing::info!("Starting SurrealDB sidecar process...");
        tracing::info!("Data path: {:?}", data_path);

        // Ensure data directory exists
        if !data_path.exists() {
            std::fs::create_dir_all(&data_path)
                .map_err(|e| format!("Failed to create data directory: {}", e))?;
        }

        // Check for and clean up stale lock file
        let db_path = data_path.join("db");
        let lock_file = db_path.join("LOCK");
        if lock_file.exists() {
            tracing::warn!("Found existing LOCK file, attempting to clean up...");
            if let Err(e) = std::fs::remove_file(&lock_file) {
                tracing::error!("Failed to remove stale lock file: {}", e);
                tracing::info!("If the problem persists, manually delete: {:?}", lock_file);
            } else {
                tracing::info!("Removed stale lock file");
            }
        }

        // Get the path to the SurrealDB binary
        // In development: look for binary in sidecar-binaries/
        // In production: Tauri will bundle it automatically
        let surreal_path = if cfg!(debug_assertions) {
            // Development: use local binary
            PathBuf::from("sidecar-binaries/surreal.exe")
        } else {
            // Production: use bundled binary
            // Tauri will handle the path resolution
            PathBuf::from("surreal.exe")
        };

        tracing::info!("SurrealDB binary path: {:?}", surreal_path);

        // Start SurrealDB process
        // surreal start --bind 127.0.0.1:8000 --user root --pass root file://data/db
        let db_file_path = data_path.join("db");
        let process = Command::new(&surreal_path)
            .args(&[
                "start",
                "--bind",
                "127.0.0.1:8000",
                "--user",
                "root",
                "--pass",
                "root",
                &format!("file://{}", db_file_path.display()),
            ])
            .spawn()
            .map_err(|e| format!("Failed to start SurrealDB: {}", e))?;

        tracing::info!("SurrealDB sidecar started (PID: {})", process.id());

        Ok(Self {
            process: Some(process),
            data_path,
        })
    }

    /// Wait for SurrealDB to be ready
    pub async fn wait_for_ready(&self, timeout_secs: u64) -> Result<(), String> {
        tracing::info!("Waiting for SurrealDB to be ready...");

        let client = reqwest::Client::new();
        let health_url = "http://127.0.0.1:8000/health";

        let start = std::time::Instant::now();
        loop {
            if start.elapsed().as_secs() > timeout_secs {
                return Err("SurrealDB failed to start within timeout".to_string());
            }

            match client.get(health_url).send().await {
                Ok(response) if response.status().is_success() => {
                    tracing::info!("✅ SurrealDB is ready!");
                    return Ok(());
                }
                _ => {
                    tracing::debug!("SurrealDB not ready yet, retrying...");
                    sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }

    /// Stop the SurrealDB sidecar
    pub fn stop(&mut self) {
        if let Some(mut process) = self.process.take() {
            let pid = process.id();
            tracing::info!("Stopping SurrealDB sidecar (PID: {})...", pid);

            // On Windows, use taskkill for graceful shutdown
            #[cfg(target_os = "windows")]
            {
                // Try graceful shutdown first
                let _ = Command::new("taskkill")
                    .args(&["/PID", &pid.to_string()])
                    .output();

                // Wait a bit for graceful shutdown
                std::thread::sleep(Duration::from_millis(500));

                // Check if still running, force kill if needed
                if let Ok(None) = process.try_wait() {
                    tracing::warn!("Process still running, forcing termination...");
                    let _ = Command::new("taskkill")
                        .args(&["/F", "/PID", &pid.to_string()])
                        .output();
                }
            }

            // On Unix, use SIGTERM then SIGKILL
            #[cfg(not(target_os = "windows"))]
            {
                let _ = process.kill();
            }

            // Wait for process to exit
            let _ = process.wait();
            tracing::info!("SurrealDB sidecar stopped");
        }
    }
}

impl Drop for SurrealDbSidecar {
    fn drop(&mut self) {
        tracing::info!("SurrealDbSidecar being dropped, ensuring cleanup...");
        self.stop();

        // Extra safety: try to remove lock file after process stops
        let lock_file = self.data_path.join("db").join("LOCK");
        if lock_file.exists() {
            std::thread::sleep(Duration::from_millis(100));
            let _ = std::fs::remove_file(&lock_file);
        }
    }
}
