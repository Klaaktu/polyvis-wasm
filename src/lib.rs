mod instance;
mod polygon;

use crate::instance::Instance;
use wasm_bindgen::prelude::*;

// I need global / static hash map. Alternative is once_cell crate.
#[wasm_bindgen]
pub fn new_session() -> Instance {
    return Instance::new();
}
