use crate::instance::Instance;
use wasm_bindgen::prelude::*;

// I need global / static hash map. Alternative is once_cell crate.
#[wasm_bindgen]
pub fn new_session() -> Instance {
    return Instance::new();
}

#[wasm_bindgen]
pub enum TextFormat {
    JSON,
    YAML,
}

#[wasm_bindgen]
pub fn serialize_session(session: &Instance, format: TextFormat) -> Result<String, String> {
    return match format {
        TextFormat::JSON => serde_json::to_string(session).map_err(|e| e.to_string()),
        TextFormat::YAML => serde_yaml_ng::to_string(session).map_err(|e| e.to_string()),
    };
}

#[wasm_bindgen]
pub fn import_session(text: &str) -> Result<Instance, String> {
    serde_json::from_str(text).map_err(|_| "Parsing input failed!".into())
}
