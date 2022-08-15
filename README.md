# rust-wasm-utils

## macros

### web_log

Internamente usa la funzione ```web_sys::console::log_1``` a cui viene passato un JsValue.

Esempi:
```rust
web_log!("Hello");
// con il prefisso f la stringa viene formattata esattamente come in format!(...)
web_log!(f"Hello from \"{}\"", "rust-wasm-utils");
```
