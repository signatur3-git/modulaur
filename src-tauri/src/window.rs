use tauri::WebviewWindow;

/// Initialize main application window
#[allow(dead_code)] // Will be used for multi-window support later
pub fn create_main_window(app_handle: &tauri::AppHandle) -> Result<WebviewWindow, tauri::Error> {
    tauri::WebviewWindowBuilder::new(
        app_handle,
        "main",
        tauri::WebviewUrl::App("index.html".into()),
    )
    .title("Modulaur - Dashboard")
    .inner_size(1200.0, 800.0)
    .min_inner_size(800.0, 600.0)
    .resizable(true)
    .build()
}
