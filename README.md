# rust-wasm-utils

## macros

### web_log

Internamente usa la funzione ```web_sys::console::log_1``` a cui viene passato un JsValue.

Esempi:
```rust
// si comporta esattamente come format!(...)
web_log!("Hello");
web_log!("Hello from \"{}\"", "rust-wasm-utils");
```

### js

Questa macro crea una istanza di JsValue.

Esempi:
```rust
// si comporta esattamente come format!(...)
let js: JsValue = js!("My js value");
let js: JsValue = js!("My {} value", "js");
```

### test
```bash
cargo watch -q -s "wasm-pack test --chrome"
```