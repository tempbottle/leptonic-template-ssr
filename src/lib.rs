use cfg_if::cfg_if;

pub mod app;
pub mod pages;

pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::prelude::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default_with_config(
            tracing_wasm::WASMLayerConfigBuilder::default()
                .set_max_level(tracing::Level::DEBUG)
                .build(),
        );

        mount_to_body(app::App);
    }
}}
