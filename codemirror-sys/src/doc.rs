use wasm_bindgen::prelude::*;
use web_sys::Element;

use crate::Editor;

#[wasm_bindgen]
extern "C" {

    #[derive(Debug)]
    pub type Doc;

    #[derive(Debug)]
    pub type LineHandle;

    #[wasm_bindgen(method, js_name = getEditor)]
    pub fn get_editor(this: &Doc) -> Editor;

    #[wasm_bindgen(method, js_name = getValue)]
    pub fn get_value(this: &Doc) -> JsValue;

    #[wasm_bindgen(method, js_name = setValue)]
    pub fn set_value(this: &Doc, text: &JsValue);

    #[wasm_bindgen(method, js_name = setGutterMarker)]
    pub fn set_gutter_marker(this: &Doc, line: &JsValue, gutter_id: &str) -> LineHandle;

    #[wasm_bindgen(method, js_name = setGutterMarker)]
    pub fn set_gutter_marker_with_element(
        this: &Doc,
        line: &JsValue,
        gutter_id: &str,
        value: &Element,
    ) -> LineHandle;

    #[wasm_bindgen(method, js_name = clearGutter)]
    pub fn clear_gutter(this: &Doc, gutter_id: &str);
}
