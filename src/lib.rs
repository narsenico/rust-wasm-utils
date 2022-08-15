#![macro_use]
#![allow(unused)]

#[macro_export]
macro_rules! web_log {
    ($($arg:tt)*) => {
        web_sys::console::log_1(&JsValue::from(format!($($arg)*)))
    };
}

#[macro_export]
macro_rules! js {
    ($($arg:tt)*) => {
        wasm_bindgen::JsValue::from(format!($($arg)*))
    };
}