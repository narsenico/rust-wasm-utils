#![macro_use]
#![allow(unused)]

#[macro_export]
macro_rules! web_log {
    ($t:tt) => {
        web_sys::console::log_1(&JsValue::from(format!($t)))
    };
    ($t:tt, $($e:expr), *) => {
        web_sys::console::log_1(&JsValue::from(format!($t, $($e), *)))
    };
}

#[macro_export]
macro_rules! js {
    ($t:tt) => {
        wasm_bindgen::JsValue::from(format!($t))
    };
    ($t:tt, $($e:expr), *) => {
        wasm_bindgen::JsValue::from(format!($t, $($e), *))
    };
}