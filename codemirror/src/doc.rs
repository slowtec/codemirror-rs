use web_sys::Element;

use crate::{GutterId, LineHandle};

pub trait DocApi {
    fn value(&self) -> Option<String>;
    fn set_value(&self, value: &str);
    fn set_gutter_marker<H>(&self, line_handle: H, id: GutterId, el: &Element) -> LineHandle
    where
        H: Into<LineHandle>;
    fn clear_gutter_marker<H>(&self, line_handle: H, id: GutterId) -> LineHandle
    where
        H: Into<LineHandle>;
    fn clear_gutter(&self, id: GutterId);
}
