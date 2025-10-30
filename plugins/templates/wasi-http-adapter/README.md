# WASI HTTP Adapter Template

A template for creating WASM plugins that use HTTP to fetch data from external APIs.

**Key Features:**
- ✅ Pure WASI - No wasm-bindgen
- ✅ HTTP via host functions - No complex dependencies
- ✅ Small binary size - Optimized for size
- ✅ Secure - Sandboxed execution

---

## 🏗️ Architecture

```
Plugin (WASM)
    ↓
    Calls: http.get(url) or http.request(...)
    ↓
Host (Rust)
    ↓
    Makes actual HTTP request via reqwest
    ↓
    Returns response to plugin
```

**Benefits:**
- Plugin doesn't need HTTP client (smaller WASM)
- Host controls all network access (security)
- Works with current wasmtime without preview2

---

## 📦 Building

### Prerequisites

```bash
# Install Rust
rustup install stable

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Build

```bash
# Linux/Mac
./build.sh

# Windows
build.bat
```

**Output:** `target/wasm32-unknown-unknown/release/wasi_http_adapter_template.wasm`

---

## 🔧 How to Use This Template

### 1. Copy the Template

```bash
cp -r plugins/templates/wasi-http-adapter plugins/examples/my-adapter
cd plugins/examples/my-adapter
```

### 2. Update Cargo.toml

```toml
[package]
name = "my-adapter"
version = "0.1.0"
```

### 3. Implement Your Adapter

Edit `src/lib.rs`:

```rust
#[no_mangle]
pub extern "C" fn fetch(config_ptr: *const c_char) -> *mut c_char {
    // 1. Parse config
    let config: YourConfig = parse_config(config_ptr);
    
    // 2. Build API URL
    let url = format!("{}/api/endpoint", config.endpoint);
    
    // 3. Make HTTP request
    let response = http_get(&url)?;
    
    // 4. Parse response and convert to StagedRecord
    let records = parse_response(&response.body)?;
    
    // 5. Return JSON
    return_json(&records)
}
```

### 4. Build

```bash
cargo build --release --target wasm32-unknown-unknown
```

### 5. Create manifest.json

```json
{
  "name": "my-adapter",
  "version": "0.1.0",
  "backend": {
    "type": "wasm",
    "entry": "backend/target/wasm32-unknown-unknown/release/my_adapter.wasm",
    "adapters": [{
      "type": "my-source",
      "name": "My Data Source",
      "capabilities": ["fetch", "test_connection"]
    }]
  }
}
```

---

## 📚 API Reference

### HTTP Host Functions

#### `http.get(url) -> HttpResponse`

Make a GET request:

```rust
let response = http_get("https://api.example.com/data")?;
println!("Status: {}", response.status);
println!("Body: {}", response.body);
```

#### `http.request(url, method, headers, body) -> HttpResponse`

Make a full HTTP request:

```rust
let mut headers = HashMap::new();
headers.insert("Authorization".to_string(), "Bearer token".to_string());

let response = http_request(
    "https://api.example.com/data",
    "POST",
    Some(&headers),
    Some(b"{\"key\":\"value\"}")
)?;
```

### HttpResponse Structure

```rust
struct HttpResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
}
```

---

## 🎯 Best Practices

### 1. Error Handling

Always handle errors gracefully:

```rust
match http_get(&url) {
    Ok(response) => {
        if response.status != 200 {
            return create_error_response(&format!("HTTP {}", response.status));
        }
        // Process response
    }
    Err(e) => return create_error_response(&format!("Request failed: {}", e)),
}
```

### 2. Authentication

Extract auth from config:

```rust
let auth = config.get("auth")
    .and_then(|a| a.get("token"))
    .and_then(|t| t.as_str())
    .ok_or("Missing auth token")?;

let mut headers = HashMap::new();
headers.insert("Authorization".to_string(), format!("Bearer {}", auth));
```

### 3. Pagination

Handle paginated APIs:

```rust
let per_page = config.get("per_page")?.as_u64()?.unwrap_or(20);
let max_pages = config.get("max_pages")?.as_u64()?.unwrap_or(1);

let mut all_records = Vec::new();

for page in 1..=max_pages {
    let url = format!("{}?page={}&per_page={}", base_url, page, per_page);
    let response = http_get(&url)?;
    let records = parse_records(&response.body)?;
    
    all_records.extend(records);
    
    if records.len() < per_page as usize {
        break; // Last page
    }
}
```

### 4. Rate Limiting

Be respectful of API rate limits:

```rust
// Add delay between requests if needed
// Note: WASI doesn't have sleep, so handle in host or limit pages
```

---

## 🔍 Debugging

### Enable Logging

The host logs WASM function calls:

```
tracing::debug!("Calling WASM function: fetch");
```

### Check Binary Size

```bash
ls -lh target/wasm32-unknown-unknown/release/*.wasm
```

**Target:** < 500KB for adapters

### Test Locally

Before deploying as plugin, test the HTTP logic in regular Rust:

```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_fetch_logic() {
        // Test your API parsing logic
    }
}
```

---

## 🚀 Examples

### GitLab Adapter

See `plugins/examples/gitlab-adapter/` for a full implementation.

### Prometheus Adapter

See `plugins/examples/prometheus-adapter/` (coming soon).

---

## 📖 Further Reading

- [WASI Overview](https://wasi.dev/)
- [Wasmtime Book](https://docs.wasmtime.dev/)
- [Plugin Development Guide](../../documentation/M7-WASI-PLUGIN-GUIDE.md)

---

**Created:** 2025-11-11  
**Status:** Template - Ready for use

