use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;

use crate::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct ChangeObject {
    pub from: Position,
    pub to: Position,
    pub text: Vec<String>,
    pub removed: Vec<String>,
    pub origin: Option<String>,
}

impl TryFrom<JsValue> for ChangeObject {
    type Error = &'static str;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let change_obj: Object = value.into();

        let from =
            Reflect::get(&change_obj, &JsValue::from("from")).map_err(|_| "Missing 'from'")?;
        let from_line = Reflect::get(&from, &JsValue::from("line"))
            .map_err(|_| "Missing 'line' in from")?
            .as_f64()
            .ok_or("Invalid number for line")? as u32;
        let from_column = Reflect::get(&from, &JsValue::from("ch"))
            .map_err(|_| "Missing 'ch' in from")?
            .as_f64()
            .ok_or("Invalid number for character")? as u32;

        let to = Reflect::get(&change_obj, &JsValue::from("to")).map_err(|_| "Missing 'to'")?;
        let to_line = Reflect::get(&to, &JsValue::from("line"))
            .map_err(|_| "Missing 'line' in to")?
            .as_f64()
            .ok_or("Invalid number for line")? as u32;
        let to_column = Reflect::get(&to, &JsValue::from("ch"))
            .map_err(|_| "Missing 'ch' in to")?
            .as_f64()
            .ok_or("Invalid number for character")? as u32;

        let text_array =
            Reflect::get(&change_obj, &JsValue::from("text")).map_err(|_| "Missing 'text'")?;
        let text: Vec<String> = Array::from(&text_array)
            .iter()
            .map(|v| v.as_string().ok_or("Invalid string in text"))
            .collect::<Result<_, _>>()?;

        let removed_array = Reflect::get(&change_obj, &JsValue::from("removed"))
            .map_err(|_| "Missing 'removed'")?;
        let removed: Vec<String> = Array::from(&removed_array)
            .iter()
            .map(|v| v.as_string().ok_or("Invalid string in removed"))
            .collect::<Result<_, _>>()?;

        let origin = Reflect::get(&change_obj, &JsValue::from("origin"))
            .ok()
            .and_then(|val| val.as_string());

        let from = Position::new(from_line.into(), from_column.into());
        let to = Position::new(to_line.into(), to_column.into());

        Ok(ChangeObject {
            from,
            to,
            text,
            removed,
            origin,
        })
    }
}
