use crate::instance::Instance;
use wasm_bindgen::prelude::*;

// Call this in JS main(). This is like main() for lib.
// I need global / static hash map. Alternative is once_cell crate.
#[wasm_bindgen]
pub fn new_session() -> Instance {
    return Instance::new();
}

#[wasm_bindgen]
pub fn import_session(text: &str) -> Result<Instance, String> {
    serde_json::from_str(text).map_err(|_| "Parsing input failed!".into())
}
