//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use rust_wasm_utils::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn macro_js() {
    // Arrange
    let expected = wasm_bindgen::JsValue::from("hello".to_string());

    // Act
    let js_value = js!("hello");

    // Assert
    assert_eq!(expected, js_value);
}

#[wasm_bindgen_test]
fn macro_js_format() {
    // Arrange
    let expected = wasm_bindgen::JsValue::from("hello world".to_string());

    // Act
    let js_value = js!("hello {}", "world");

    // Assert
    assert_eq!(expected, js_value);
}

#[wasm_bindgen_test]
fn macro_js_expr() {
    // Arrange
    let expected = wasm_bindgen::JsValue::from("200".to_string());

    // Act
    let js_value: wasm_bindgen::JsValue = js!("{}", 100 + 100);

    // Assert
    assert_eq!(expected, js_value);
}
