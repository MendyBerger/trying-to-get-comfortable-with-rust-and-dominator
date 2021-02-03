use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "export function log_json(s) { console.log(s) }")]
// #[wasm_bindgen(inline_js = "export function log_json(s) { console.log(JSON.parse(s)) }")]
extern "C" {
    fn log_json(obj: &str);
}

pub fn log<T>(value: &T)
where
    T: ?Sized + Serialize,
{
    log_json(&serde_json::to_string_pretty(value).unwrap_or("can't".to_string()))
}
