///////////////Logging
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn wasm_log(s: &str);
}

macro_rules! log {
    ($($t:tt)*) => (crate::log::wasm_log(&format_args!($($t)*).to_string()))
}

pub(crate) use log;
