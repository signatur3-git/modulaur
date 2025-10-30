// HTTP Host Functions for WASM Plugins
//
// Provides HTTP capabilities to WASM plugins via host functions.
// This allows plugins to make HTTP requests without needing WASI HTTP support.

use std::str;
use wasmtime::*;
use wasmtime_wasi::preview1::WasiP1Ctx;

/// Add HTTP host functions to the linker
///
/// This provides:
/// - http_request: Make HTTP requests (GET, POST, etc.)
/// - http_get: Simplified GET request
/// - http_post: Simplified POST request
pub fn add_http_to_linker(linker: &mut Linker<WasiP1Ctx>) -> Result<(), anyhow::Error> {
    // http_request: Full HTTP request with all options
    linker.func_wrap(
        "http",
        "request",
        |mut caller: Caller<'_, WasiP1Ctx>,
         url_ptr: i32,
         url_len: i32,
         method_ptr: i32,
         method_len: i32,
         headers_ptr: i32,
         headers_len: i32,
         body_ptr: i32,
         body_len: i32,
         result_ptr_ptr: i32|
         -> i32 {
            // Get memory from caller
            let memory = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => return -1, // Error: no memory export
            };

            // Read URL from WASM memory
            let url =
                match read_string_from_memory(&caller, &memory, url_ptr as usize, url_len as usize)
                {
                    Ok(s) => s,
                    Err(_) => return -1,
                };

            // Read method from WASM memory
            let method = match read_string_from_memory(
                &caller,
                &memory,
                method_ptr as usize,
                method_len as usize,
            ) {
                Ok(s) => s,
                Err(_) => return -1,
            };

            // Read headers JSON from WASM memory (if provided)
            let headers_json = if headers_len > 0 {
                match read_string_from_memory(
                    &caller,
                    &memory,
                    headers_ptr as usize,
                    headers_len as usize,
                ) {
                    Ok(s) => Some(s),
                    Err(_) => return -1,
                }
            } else {
                None
            };

            // Read body from WASM memory (if provided)
            let body = if body_len > 0 {
                match read_bytes_from_memory(&caller, &memory, body_ptr as usize, body_len as usize)
                {
                    Ok(b) => Some(b),
                    Err(_) => return -1,
                }
            } else {
                None
            };

            // Make HTTP request (sync - we'll use tokio::runtime for async)
            let result = match make_http_request_sync(
                &url,
                &method,
                headers_json.as_deref(),
                body.as_deref(),
            ) {
                Ok(response_json) => response_json,
                Err(e) => {
                    eprintln!("HTTP request failed: {}", e);
                    return -1;
                }
            };

            // Allocate space in WASM memory for result
            // Try to call the plugin's allocator if it exists
            let result_bytes = result.as_bytes();
            let result_len = result_bytes.len() as i32;

            let alloc_fn: TypedFunc<i32, i32> = match caller.get_export("alloc") {
                Some(Extern::Func(func)) => match func.typed(&caller) {
                    Ok(f) => f,
                    Err(_) => return -1,
                },
                _ => return -1, // No allocator
            };

            let result_ptr = match alloc_fn.call(&mut caller, result_len + 1) {
                Ok(ptr) => ptr,
                Err(_) => return -1,
            };

            // Write result to WASM memory
            if memory
                .write(&mut caller, result_ptr as usize, result_bytes)
                .is_err()
            {
                return -1;
            }

            // Write null terminator
            if memory
                .write(
                    &mut caller,
                    (result_ptr as usize) + result_bytes.len(),
                    &[0],
                )
                .is_err()
            {
                return -1;
            }

            // Write result pointer to the output pointer location
            let ptr_bytes = (result_ptr as u32).to_le_bytes();
            if memory
                .write(&mut caller, result_ptr_ptr as usize, &ptr_bytes)
                .is_err()
            {
                return -1;
            }

            result_len
        },
    )?;

    // http_get: Simplified GET request
    linker.func_wrap(
        "http",
        "get",
        |mut caller: Caller<'_, WasiP1Ctx>,
         url_ptr: i32,
         url_len: i32,
         result_ptr_ptr: i32|
         -> i32 {
            // Get memory
            let memory = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => return -1,
            };

            // Read URL
            let url =
                match read_string_from_memory(&caller, &memory, url_ptr as usize, url_len as usize)
                {
                    Ok(s) => s,
                    Err(_) => return -1,
                };

            // Make GET request
            let result = match make_http_request_sync(&url, "GET", None, None) {
                Ok(response_json) => response_json,
                Err(e) => {
                    eprintln!("HTTP GET failed: {}", e);
                    return -1;
                }
            };

            // Allocate and write result
            let result_bytes = result.as_bytes();
            let result_len = result_bytes.len() as i32;

            let alloc_fn: TypedFunc<i32, i32> = match caller.get_export("alloc") {
                Some(Extern::Func(func)) => match func.typed(&caller) {
                    Ok(f) => f,
                    Err(_) => return -1,
                },
                _ => return -1,
            };

            let result_ptr = match alloc_fn.call(&mut caller, result_len + 1) {
                Ok(ptr) => ptr,
                Err(_) => return -1,
            };

            if memory
                .write(&mut caller, result_ptr as usize, result_bytes)
                .is_err()
            {
                return -1;
            }
            if memory
                .write(
                    &mut caller,
                    (result_ptr as usize) + result_bytes.len(),
                    &[0],
                )
                .is_err()
            {
                return -1;
            }

            // Write result pointer
            let ptr_bytes = (result_ptr as u32).to_le_bytes();
            if memory
                .write(&mut caller, result_ptr_ptr as usize, &ptr_bytes)
                .is_err()
            {
                return -1;
            }

            result_len
        },
    )?;

    Ok(())
}

/// Read a string from WASM memory
fn read_string_from_memory(
    caller: &Caller<'_, WasiP1Ctx>,
    memory: &Memory,
    ptr: usize,
    len: usize,
) -> Result<String, anyhow::Error> {
    let mut buffer = vec![0u8; len];
    memory.read(caller, ptr, &mut buffer)?;
    Ok(String::from_utf8(buffer)?)
}

/// Read bytes from WASM memory
fn read_bytes_from_memory(
    caller: &Caller<'_, WasiP1Ctx>,
    memory: &Memory,
    ptr: usize,
    len: usize,
) -> Result<Vec<u8>, anyhow::Error> {
    let mut buffer = vec![0u8; len];
    memory.read(caller, ptr, &mut buffer)?;
    Ok(buffer)
}

/// Make HTTP request synchronously (blocks on async)
fn make_http_request_sync(
    url: &str,
    method: &str,
    headers_json: Option<&str>,
    body: Option<&[u8]>,
) -> Result<String, anyhow::Error> {
    // Use block_in_place to safely block within an async runtime
    // This moves the blocking operation to a blocking thread pool
    tokio::task::block_in_place(|| {
        // Create a new Handle to the current runtime
        let handle = tokio::runtime::Handle::current();

        // Use the handle to spawn the async work
        handle.block_on(async {
            let client = reqwest::Client::new();

            let mut request = match method.to_uppercase().as_str() {
                "GET" => client.get(url),
                "POST" => client.post(url),
                "PUT" => client.put(url),
                "DELETE" => client.delete(url),
                "PATCH" => client.patch(url),
                _ => return Err(anyhow::anyhow!("Unsupported HTTP method: {}", method)),
            };

            // Add headers if provided
            if let Some(headers_str) = headers_json {
                if let Ok(headers_map) =
                    serde_json::from_str::<std::collections::HashMap<String, String>>(headers_str)
                {
                    for (key, value) in headers_map {
                        request = request.header(key, value);
                    }
                }
            }

            // Add body if provided
            if let Some(body_data) = body {
                request = request.body(body_data.to_vec());
            }

            // Send request
            let response = request.send().await?;
            let status = response.status();
            let headers = response.headers().clone();
            let body = response.text().await?;

            // Create response JSON
            let response_json = serde_json::json!({
                "status": status.as_u16(),
                "headers": headers.iter()
                    .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
                    .collect::<std::collections::HashMap<_, _>>(),
                "body": body,
            });

            Ok(serde_json::to_string(&response_json)?)
        })
    })
}
