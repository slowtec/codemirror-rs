use codemirror::{DocApi, Editor, EditorOptions};
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlTextAreaElement};

#[wasm_bindgen(start)]
fn main() {
    let document = window().unwrap().document().unwrap();
    let text_area = document
        .create_element("textarea")
        .unwrap()
        .dyn_into::<HtmlTextAreaElement>()
        .unwrap();
    document.body().unwrap().append_child(&text_area).unwrap();

    let options = EditorOptions::default().line_numbers(true);
    let editor = Editor::from_text_area(&text_area, &options);

    editor.set_value("Hello from Rust");

    editor.on_change(|_editor, _change| {
        // value changed
    });
}
