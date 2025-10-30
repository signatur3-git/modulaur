// WASI HTTP Adapter Template
//
// This template demonstrates how to create a WASM plugin that uses
// HTTP host functions to fetch data from external APIs.
//
// NO wasm-bindgen! Pure WASI with host function calls.

use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// ============================================================================
// HTTP Host Functions (provided by the host)
// ============================================================================

#[link(wasm_import_module = "http")]
extern "C" {
    /// Make an HTTP GET request
    /// Returns length of response, writes pointer to result_ptr_ptr
    fn get(url_ptr: *const c_char, url_len: i32, result_ptr_ptr: *mut i32) -> i32;

    /// Make a full HTTP request with custom method, headers, and body
    fn request(
        url_ptr: *const c_char,
        url_len: i32,
        method_ptr: *const c_char,
        method_len: i32,
        headers_ptr: *const c_char,
        headers_len: i32,
        body_ptr: *const u8,
        body_len: i32,
        result_ptr_ptr: *mut i32,
    ) -> i32;
}

// ============================================================================
// Memory Management
// ============================================================================

/// Allocate memory in WASM (called by host)
#[no_mangle]
pub extern "C" fn alloc(size: i32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// Free a string allocated by the plugin
#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Make an HTTP GET request using the host function
fn http_get(url: &str) -> Result<HttpResponse, String> {
    let url_cstring = CString::new(url).map_err(|e| format!("Invalid URL: {}", e))?;
    let url_ptr = url_cstring.as_ptr();
    let url_len = url.len() as i32;

    let mut result_ptr: i32 = 0;

    unsafe {
        let result_len = get(url_ptr, url_len, &mut result_ptr as *mut i32);

        if result_len < 0 {
            return Err("HTTP GET request failed".to_string());
        }

        // Read response from memory
        let response_slice = std::slice::from_raw_parts(
            result_ptr as *const u8,
            result_len as usize,
        );
        let response_str = std::str::from_utf8(response_slice)
            .map_err(|e| format!("Invalid UTF-8 in response: {}", e))?;

        // Parse HTTP response
        let response: HttpResponse = serde_json::from_str(response_str)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(response)
    }
}

/// Make a full HTTP request using the host function
#[allow(dead_code)]
fn http_request(
    url: &str,
    method: &str,
    headers: Option<&std::collections::HashMap<String, String>>,
    body: Option<&[u8]>,
) -> Result<HttpResponse, String> {
    let url_cstring = CString::new(url).map_err(|e| format!("Invalid URL: {}", e))?;
    let method_cstring = CString::new(method).map_err(|e| format!("Invalid method: {}", e))?;

    let headers_json = if let Some(h) = headers {
        serde_json::to_string(h).map_err(|e| format!("Failed to serialize headers: {}", e))?
    } else {
        String::new()
    };
    let headers_cstring = CString::new(headers_json).unwrap_or_default();

    let mut result_ptr: i32 = 0;

    unsafe {
        let result_len = request(
            url_cstring.as_ptr(),
            url.len() as i32,
            method_cstring.as_ptr(),
            method.len() as i32,
            headers_cstring.as_ptr(),
            headers_cstring.to_bytes().len() as i32,
            body.map(|b| b.as_ptr()).unwrap_or(std::ptr::null()),
            body.map(|b| b.len()).unwrap_or(0) as i32,
            &mut result_ptr as *mut i32,
        );

        if result_len < 0 {
            return Err("HTTP request failed".to_string());
        }

        // Read and parse response
        let response_slice = std::slice::from_raw_parts(
            result_ptr as *const u8,
            result_len as usize,
        );
        let response_str = std::str::from_utf8(response_slice)
            .map_err(|e| format!("Invalid UTF-8 in response: {}", e))?;

        let response: HttpResponse = serde_json::from_str(response_str)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(response)
    }
}

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Deserialize)]
struct HttpResponse {
    status: u16,
    headers: std::collections::HashMap<String, String>,
    body: String,
}

// ============================================================================
// Plugin Functions (called by host)
// ============================================================================

/// Fetch data from the adapter
/// Input: JSON config
/// Output: JSON array of records
#[no_mangle]
pub extern "C" fn fetch(config_ptr: *const c_char) -> *mut c_char {
    // Parse config
    let config_str = unsafe {
        if config_ptr.is_null() {
            return create_error_response("Null config pointer");
        }
        match CStr::from_ptr(config_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return create_error_response("Invalid UTF-8 in config"),
        }
    };

    // Deserialize config
    let config: serde_json::Value = match serde_json::from_str(config_str) {
        Ok(c) => c,
        Err(e) => return create_error_response(&format!("Failed to parse config: {}", e)),
    };

    // Extract endpoint
    let endpoint = match config.get("endpoint").and_then(|e| e.as_str()) {
        Some(e) => e,
        None => return create_error_response("Missing endpoint in config"),
    };

    // TODO: Extract authentication
    // let auth = config.get("auth");

    // TODO: Build API URL based on your adapter's needs
    // Example: let url = format!("{}/api/data", endpoint);

    // Make HTTP request
    let response = match http_get(endpoint) {
        Ok(r) => r,
        Err(e) => return create_error_response(&format!("HTTP request failed: {}", e)),
    };

    // Check status
    if response.status != 200 {
        return create_error_response(&format!("HTTP error: {}", response.status));
    }

    // TODO: Parse response.body and convert to StagedRecord format
    // For now, return empty array
    let records: Vec<serde_json::Value> = vec![];

    let result_json = match serde_json::to_string(&records) {
        Ok(j) => j,
        Err(e) => return create_error_response(&format!("Failed to serialize result: {}", e)),
    };

    // Return as C string
    match CString::new(result_json) {
        Ok(s) => s.into_raw(),
        Err(_) => create_error_response("Failed to create result string"),
    }
}

/// Test connection to the adapter
#[no_mangle]
pub extern "C" fn test_connection(config_ptr: *const c_char) -> *mut c_char {
    let config_str = unsafe {
        if config_ptr.is_null() {
            return create_error_response("Null config pointer");
        }
        match CStr::from_ptr(config_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return create_error_response("Invalid UTF-8 in config"),
        }
    };

    let config: serde_json::Value = match serde_json::from_str(config_str) {
        Ok(c) => c,
        Err(e) => return create_error_response(&format!("Failed to parse config: {}", e)),
    };

    let endpoint = match config.get("endpoint").and_then(|e| e.as_str()) {
        Some(e) => e,
        None => return create_error_response("Missing endpoint in config"),
    };

    // Try to connect
    match http_get(endpoint) {
        Ok(_) => {
            let success = serde_json::json!({"success": true});
            match CString::new(success.to_string()) {
                Ok(s) => s.into_raw(),
                Err(_) => create_error_response("Failed to create response"),
            }
        }
        Err(e) => create_error_response(&format!("Connection failed: {}", e)),
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn create_error_response(message: &str) -> *mut c_char {
    let error = serde_json::json!({
        "error": message
    });

    CString::new(error.to_string())
        .unwrap_or_else(|_| CString::new("Unknown error").unwrap())
        .into_raw()
}

