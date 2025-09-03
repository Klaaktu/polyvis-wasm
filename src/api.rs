use crate::instance::Instance;
use wasm_bindgen::prelude::*;
use web_sys::{Blob, HtmlAnchorElement, Url};

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
pub fn export_session(session: &Instance, format: TextFormat) -> Result<(), String> {
    let text = match format {
        TextFormat::JSON => serde_json::to_string(session).map_err(|e| e.to_string())?,
        TextFormat::YAML => serde_yaml_ng::to_string(session).map_err(|e| e.to_string())?,
    };
    let ext = match format {
        TextFormat::JSON => ".json",
        TextFormat::YAML => ".yaml",
    };

    let document = web_sys::window()
        .ok_or("No global window exists!")?
        .document()
        .ok_or("Should have a document on window!")?;

    let blob = Blob::new_with_str_sequence(&JsValue::from_str(&text))
        .map_err(|_| "Blob creation failed.")?;
    let link: HtmlAnchorElement = document
        .create_element("temp")
        .map_err(|_| "HTML element creation failed.")?
        .dyn_into()
        .map_err(|_| "Type cast from Element to HTMLAnchorElement failed!")?;
    let url =
        Url::create_object_url_with_blob(&blob).map_err(|_| "URL creation for blob failed!")?;
    link.set_href(&url);
    link.set_download(&format!("save.{ext}"));
    link.click();
    Url::revoke_object_url(&url).map_err(|_| "URL removal for blob failed!")?;
    return Ok(());
}

#[wasm_bindgen]
pub fn import_session(text: &str) -> Result<Instance, String> {
    serde_json::from_str(text).map_err(|_| "Parsing input failed!".into())
}
