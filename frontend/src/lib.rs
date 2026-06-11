pub mod app;
pub mod components;
pub mod content;
pub mod islands;
pub mod pages;

#[cfg(all(test, feature = "ssr"))]
mod ssr_tests;

/// Browser entry point. In islands mode only `#[island]` components are
/// shipped as wasm and hydrated; everything else stays server-rendered HTML.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
