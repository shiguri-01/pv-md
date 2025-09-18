use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
mod app;
use app::App;
pub(crate) mod components;

#[wasm_bindgen]
pub fn start_app() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App /> });
}
