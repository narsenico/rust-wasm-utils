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

pub mod convert {
    use std::{error::Error, fmt::Display};

    use js_sys::Uint8Array;
    use wasm_bindgen::JsValue;

    pub trait ToJsValue {
        fn to_js(&self) -> JsValue;
    }

    impl ToJsValue for Vec<u8> {
        fn to_js(&self) -> JsValue {
            let buffer = Uint8Array::new_with_length(self.len() as u32);
            buffer.copy_from(self.as_slice());
            JsValue::from(buffer)
        }
    }

    impl ToJsValue for String {
        fn to_js(&self) -> JsValue {
            JsValue::from(self)
        }
    }

    impl ToJsValue for str {
        fn to_js(&self) -> JsValue {
            JsValue::from_str(self)
        }
    }

    pub trait DisplayToJsValue<T> where T: Display {
        fn to_js(&self) -> JsValue;
    }

    impl<T> DisplayToJsValue<T> for T where T: Error {
        fn to_js(&self) -> JsValue {
            JsValue::from(format!("{}", self))
        }
    }

}
