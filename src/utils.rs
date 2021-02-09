use serde::Serialize;
use wasm_bindgen::prelude::*;
use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

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


pub fn get_random_string(length: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(length)
        .collect()
}