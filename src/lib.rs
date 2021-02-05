use wasm_bindgen::prelude::*;
use crate::app::App;

mod state;
mod app;
mod components;
mod db_interface;
mod utils;

#[wasm_bindgen(start)]
pub async fn main_js() {
    dominator::append_dom(&dominator::get_id("app"), App::render().await);
}
