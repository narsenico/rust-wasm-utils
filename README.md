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

## convert

I trait ```ToJsValue``` e ```DisplayToJsValue``` mettono a disposizione il metodo ```to_js()``` per convertire un oggetto in ```JsValue```.

Esempi:
```rust
let js: JsValue = "My js value".to_js();

let _ = read_to_string(reader).map_err(|err| err.to_js());
```

## test
```bash
cargo watch -q -s "wasm-pack test --chrome"
```