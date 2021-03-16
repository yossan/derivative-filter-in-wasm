mod utils;
mod image;

use crate::image::{filter_image, Error};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn load_data(data: &[u8]) -> String {
    let result = filter_image(data);
    match result {
        Ok(data) => data,
        Err(_) => "".to_string(),
    }
}
