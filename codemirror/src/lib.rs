use derive_more::From;
use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlTextAreaElement};

use codemirror_sys as sys;

#[derive(Default, Debug, Clone)]
pub struct Options {
    pub line_numbers: bool,
    pub gutters: &'static [GutterId],
}

impl Options {
    pub fn line_numbers(mut self, v: bool) -> Self {
        self.line_numbers = v;
        self
    }

    pub fn gutters(mut self, v: &'static [GutterId]) -> Self {
        self.gutters = v;
        self
    }
}

impl From<&Options> for JsValue {
    fn from(from: &Options) -> Self {
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

#[derive(Debug)]
pub struct Doc(sys::Doc);

impl Doc {
    #[must_use]
    pub fn from_text_area(text_area: &HtmlTextAreaElement, options: &Options) -> Doc {
        let doc = sys::from_text_area(text_area, &JsValue::from(options));
        Self(doc)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From)]
pub struct Line(u64);

impl Line {
    pub const fn new(line: u64) -> Self {
        Self(line)
    }
}

#[derive(Debug, From)]
pub enum LineHandle {
    Line(Line),
    Handle(sys::LineHandle),
}

impl From<LineHandle> for JsValue {
    fn from(from: LineHandle) -> Self {
        match from {
            LineHandle::Line(line) => JsValue::from(line.0 as f64),
            LineHandle::Handle(handle) => JsValue::from(handle),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, From, Hash)]
pub struct GutterId(&'static str);

impl GutterId {
    pub const fn new(id: &'static str) -> Self {
        Self(id)
    }
}

impl Doc {
    pub fn value(&self) -> Option<String> {
        self.0.get_value().as_string()
    }

    pub fn set_value(&self, value: &str) {
        self.0.set_value(&value.into())
    }

    pub fn set_gutter_marker<H>(&self, line_handle: H, id: GutterId, el: &Element) -> LineHandle
    where
        H: Into<LineHandle>,
    {
        let handle: LineHandle = line_handle.into();
        let js_handle: JsValue = handle.into();
        self.0
            .set_gutter_marker_with_element(&js_handle, id.0, el)
            .into()
    }

    pub fn clear_gutter_marker<H>(&self, line_handle: H, id: GutterId) -> LineHandle
    where
        H: Into<LineHandle>,
    {
        let handle: LineHandle = line_handle.into();
        self.0
            .set_gutter_marker(&JsValue::from(handle), id.0)
            .into()
    }

    pub fn clear_gutter(&self, id: GutterId) {
        self.0.clear_gutter(id.0)
    }
}
