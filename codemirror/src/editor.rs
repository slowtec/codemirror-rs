use derive_more::From;
use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlTextAreaElement};

use codemirror_sys as sys;

use crate::{ChangeObject, DocApi, GutterId, LineHandle};

#[derive(Debug)]
pub struct Editor(sys::Editor);

impl Editor {
    #[must_use]
    pub fn from_text_area(text_area: &HtmlTextAreaElement, options: &EditorOptions) -> Self {
        let editor = sys::from_text_area(text_area, &JsValue::from(options));
        Self(editor)
    }

    pub fn on<F>(&self, event_name: &str, mut callback: F)
    where
        F: FnMut(Editor, JsValue) + 'static,
    {
        let closure = Closure::wrap(Box::new(move |instance: sys::Editor, value: JsValue| {
            let editor = Editor(instance);
            callback(editor, value)
        }) as Box<dyn FnMut(sys::Editor, JsValue)>);
        self.0.on(event_name, closure.as_ref().unchecked_ref());
        closure.forget();
    }

    pub fn on_change<F>(&self, mut callback: F)
    where
        F: FnMut(Editor, ChangeObject) + 'static,
    {
        self.on("change", move |editor, js_value| {
            let change_obj = ChangeObject::try_from(js_value).unwrap();
            callback(editor, change_obj)
        })
    }
}

#[derive(Default, Debug, Clone)]
pub struct EditorOptions {
    pub line_numbers: bool,
    pub gutters: &'static [GutterId],
}

impl EditorOptions {
    pub fn line_numbers(mut self, v: bool) -> Self {
        self.line_numbers = v;
        self
    }

    pub fn gutters(mut self, v: &'static [GutterId]) -> Self {
        self.gutters = v;
        self
    }
}

impl From<&EditorOptions> for JsValue {
    fn from(from: &EditorOptions) -> Self {
        let options = Object::new();
        let gutters = from
            .gutters
            .iter()
            .map(|g| JsValue::from_str(g.0))
            .collect::<Vec<_>>();
        Reflect::set(&options, &"lineNumbers".into(), &from.line_numbers.into()).unwrap();
        Reflect::set(&options, &"gutters".into(), &JsValue::from(gutters)).unwrap();
        options.into()
    }
}

impl DocApi for Editor {
    fn value(&self) -> Option<String> {
        self.0.get_value().as_string()
    }

    fn set_value(&self, value: &str) {
        self.0.set_value(&value.into())
    }

    fn set_gutter_marker<H>(&self, line_handle: H, id: GutterId, el: &Element) -> LineHandle
    where
        H: Into<LineHandle>,
    {
        let handle: LineHandle = line_handle.into();
        let js_handle: JsValue = handle.into();
        self.0
            .set_gutter_marker_with_element(&js_handle, id.0, el)
            .into()
    }

    fn clear_gutter_marker<H>(&self, line_handle: H, id: GutterId) -> LineHandle
    where
        H: Into<LineHandle>,
    {
        let handle: LineHandle = line_handle.into();
        self.0
            .set_gutter_marker(&JsValue::from(handle), id.0)
            .into()
    }

    fn clear_gutter(&self, id: GutterId) {
        self.0.clear_gutter(id.0)
    }
}
