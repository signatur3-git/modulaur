use std::collections::HashMap;
use std::sync::Mutex;

/// Simple in-memory credential storage
/// In production, this could integrate with OS keychain (Windows Credential Manager, etc.)
static CREDENTIAL_STORE: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

/// Initialize the credential store
fn ensure_store() {
    let mut store = CREDENTIAL_STORE.lock().unwrap();
    if store.is_none() {
        *store = Some(HashMap::new());
    }
}

/// Store a credential securely
/// In production: Would use Windows Credential Manager API
#[tauri::command]
pub fn store_secure_credential(key: String, value: String) -> Result<(), String> {
    ensure_store();

    let mut store = CREDENTIAL_STORE.lock().unwrap();
    let map = store.as_mut().unwrap();
    map.insert(key, value);

    Ok(())
}

/// Retrieve a credential securely
/// In production: Would use Windows Credential Manager API
#[tauri::command]
pub fn get_secure_credential(key: String) -> Result<Option<String>, String> {
    ensure_store();

    let store = CREDENTIAL_STORE.lock().unwrap();
    let map = store.as_ref().unwrap();
    Ok(map.get(&key).cloned())
}

/// Remove a credential
#[tauri::command]
pub fn remove_secure_credential(key: String) -> Result<(), String> {
    ensure_store();

    let mut store = CREDENTIAL_STORE.lock().unwrap();
    let map = store.as_mut().unwrap();
    map.remove(&key);

    Ok(())
}

/// Get machine-specific password for encryption
/// In production: Could use hardware-based keys or OS key derivation
#[tauri::command]
pub fn get_machine_password() -> Result<String, String> {
    // In production, this would:
    // 1. Use Windows Credential Manager to get/create a key
    // 2. Or use DPAPI (Data Protection API)
    // 3. Or use hardware security module

    // For now, generate a consistent machine-specific value
    use std::env;

    let machine_id = format!(
        "{}{}{}",
        env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".to_string()),
        env::var("USERNAME").unwrap_or_else(|_| "user".to_string()),
        env::var("USERDOMAIN").unwrap_or_else(|_| "domain".to_string())
    );

    // Hash it to create a password
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    machine_id.hash(&mut hasher);
    let hash = hasher.finish();

    Ok(format!("{:x}", hash))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_retrieve() {
        let key = "test_key".to_string();
        let value = "secret_value".to_string();

        // Store
        store_secure_credential(key.clone(), value.clone()).unwrap();

        // Retrieve
        let retrieved = get_secure_credential(key.clone()).unwrap();
        assert_eq!(retrieved, Some(value));

        // Remove
        remove_secure_credential(key.clone()).unwrap();

        // Verify removed
        let retrieved = get_secure_credential(key).unwrap();
        assert_eq!(retrieved, None);
    }

    #[test]
    fn test_machine_password() {
        let password = get_machine_password().unwrap();
        assert!(!password.is_empty());

        // Should be consistent
        let password2 = get_machine_password().unwrap();
        assert_eq!(password, password2);
    }
}
