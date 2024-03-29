pub mod app;
mod chart;
pub mod components;
#[cfg(feature = "ssr")]
pub mod my_scheduler;
pub mod utils;
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
pub mod api;
}
}
cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(move || {
          view! {  <App/> }
      });
    }
}
}
