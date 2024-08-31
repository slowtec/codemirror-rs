use codemirror::{DocApi, Editor, EditorOptions, GutterId, Line};
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element, HtmlTextAreaElement};

const GUTTER_ERROR: GutterId = GutterId::new("gutter-error");

#[wasm_bindgen(start)]
fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    log::info!("Start web application");

    let document = document();
    let text_area = create_text_area(&document);

    document.body().unwrap().append_child(&text_area).unwrap();

    let options = EditorOptions::default()
        .line_numbers(true)
        .gutters(&[GUTTER_ERROR]);

    log::debug!("Create editor");
    let editor = Editor::from_text_area(&text_area, &options);

    editor.set_value("Hello from Rust\n\n\n");

    let marker = create_error_marker(&document);
    let line = Line::new(0);
    editor.set_gutter_marker(line, GUTTER_ERROR, &marker);

    let marker = create_error_marker(&document);
    let line = Line::new(3);
    editor.set_gutter_marker(line, GUTTER_ERROR, &marker);
    editor.on_change(|editor, change| {
        log::debug!("Document changed: {change:?}");
        log::debug!("New text is: {:?}", editor.value());
    });
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
