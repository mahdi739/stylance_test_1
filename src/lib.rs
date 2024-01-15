use stylance::import_style;
use wasm_bindgen::prelude::wasm_bindgen;
use app::App;
use leptos::*;
mod app;
mod components;
mod core;
mod env;

#[wasm_bindgen]
pub fn main() {
    import_style!(style, "main.module.css");

    leptos::mount_to_body(App);
}
