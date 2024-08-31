use wasm_bindgen::prelude::*;
use web_sys::HtmlTextAreaElement;

use crate::Doc;

#[wasm_bindgen]
extern "C" {

    #[derive(Debug)]
    #[wasm_bindgen(extends = Doc)]
    pub type Editor;

    #[wasm_bindgen(method, js_name = getDoc)]
    pub fn get_doc(this: &Editor) -> Doc;

    #[wasm_bindgen(method)]
    pub fn save(this: &Editor);

    #[wasm_bindgen(js_name = fromTextArea, js_namespace = CodeMirror)]
    pub fn from_text_area(text_area: &HtmlTextAreaElement, options: &JsValue) -> Editor;

    #[wasm_bindgen(method, js_name = on)]
    pub fn on(this: &Editor, event_name: &str, callback: &JsValue);

}
