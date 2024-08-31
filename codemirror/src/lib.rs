#![doc = include_str!("../README.md")]

use derive_more::From;
use wasm_bindgen::prelude::*;

use codemirror_sys as sys;

mod change_object;
mod doc;
mod editor;
mod position;

pub use self::{change_object::*, doc::*, editor::*, position::*};

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
