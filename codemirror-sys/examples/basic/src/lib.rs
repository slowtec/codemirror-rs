use codemirror_sys::from_text_area;
use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element, HtmlTextAreaElement};

const GUTTER_ERROR: &str = "gutter-error";

#[wasm_bindgen(start)]
fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    log::info!("Start web application");

    let document = document();
    let text_area = create_text_area(&document);

    document.body().unwrap().append_child(&text_area).unwrap();

    let gutters = JsValue::from(vec![JsValue::from(GUTTER_ERROR)]);

    let options = Object::new();
    Reflect::set(&options, &"lineNumbers".into(), &true.into()).unwrap();
    Reflect::set(&options, &"gutters".into(), &JsValue::from(gutters)).unwrap();

    log::debug!("Create editor");
    let editor = from_text_area(&text_area, &options);

    editor.set_value(&"Hello from Rust\n\n\n".into());

    let marker = create_error_marker(&document);
    let line = JsValue::from(0);
    editor.set_gutter_marker_with_element(&line, &GUTTER_ERROR, &marker);

    let marker = create_error_marker(&document);
    let line = JsValue::from(3);
    editor.set_gutter_marker_with_element(&line, &GUTTER_ERROR, &marker);
}

fn create_text_area(document: &Document) -> HtmlTextAreaElement {
    log::debug!("Create text area");
    let text_area = document.create_element("textarea").unwrap();
    text_area.dyn_into::<HtmlTextAreaElement>().unwrap()
}

const ERROR_MARKER_CLASS: &str = "CodeMirror-lint-marker-error CodeMirror-lint-marker";

fn create_error_marker(document: &Document) -> Element {
    log::debug!("Create error marker");
    let marker = document.create_element("div").unwrap();
    marker.set_attribute("class", ERROR_MARKER_CLASS).unwrap();
    marker
}

fn document() -> Document {
    window().unwrap().document().unwrap()
}
