//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::convert::From;
use std::fmt::{self, Display};
use std::format;

use rust_wasm_utils::convert::*;
use rust_wasm_utils::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn macro_js() {
    // Arrange
    let expected = JsValue::from("hello".to_string());

    // Act
    let js_value = js!("hello");

    // Assert
    assert_eq!(expected, js_value);
}

#[wasm_bindgen_test]
fn macro_js_from_string() {
    // Arrange
    let value = "hello";
    let expected = JsValue::from_str(value);

    // Act
    let js_value = js!("{}", value);

    // Assert
    assert_eq!(expected, js_value);
}

#[wasm_bindgen_test]
fn macro_js_format() {
    // Arrange
    let expected = JsValue::from("hello world".to_string());

    // Act
    let js_value = js!("hello {}", "world");

    // Assert
    assert_eq!(expected, js_value);
}

#[wasm_bindgen_test]
fn macro_js_expr() {
    // Arrange
    let expected = JsValue::from("200".to_string());

    // Act
    let js_value = js!("{}", 100 + 100);

    // Assert
    assert_eq!(expected, js_value);
}

#[wasm_bindgen_test]
fn convert_string_to_js() {
    // Arrange
    let expected = JsValue::from(String::from("hello"));

    // Act
    let js_value = String::from("hello").to_js();

    // Assert
    assert_eq!(expected, js_value);
}

#[wasm_bindgen_test]
fn convert_str_to_js() {
    // Arrange
    let expected = JsValue::from_str("hello");

    // Act
    let js_value = "hello".to_js();

    // Assert
    assert_eq!(expected, js_value);
}

#[derive(Debug)]
struct MyError {
    pub inner: String,
}
impl Display for MyError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.inner)
    }
}
impl std::error::Error for MyError {}

#[wasm_bindgen_test]
fn convert_my_error_to_js() {
    // Arrange
    let error = MyError {
        inner: "fake error".to_string(),
    };
    let expected = JsValue::from("fake error".to_string());

    // Act
    let js_value = error.to_js();

    // Assert
    assert_eq!(expected, js_value);
}

#[wasm_bindgen_test]
fn convert_io_error_to_js() {
    // Arrange
    let error = std::io::Error::from(std::io::ErrorKind::AddrInUse);
    let expected = JsValue::from(format!("{}", error));

    // Act
    let js_value = error.to_js();

    // Assert
    assert_eq!(expected, js_value);
}
