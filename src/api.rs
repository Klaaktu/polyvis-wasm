use crate::instance::Instance;
use geo::{Coord, IsConvex, LineString, coord};
use wasm_bindgen::prelude::*;

// Call this in JS main(). This is like main() for lib.
// I need global / static hash map. Alternative is once_cell crate.
#[wasm_bindgen]
pub fn new_session() -> Instance {
    return Instance::new();
}

#[wasm_bindgen]
pub fn deserialize_session(text: &str) -> Result<Instance, String> {
    serde_json::from_str(text).map_err(|_| "Parsing input failed!".into())
}

// Wrapper struct for geo::Coord. wasm_bindgen cannot export crate items.
// wasm_bindgen can only export struct not tuple or array
#[wasm_bindgen]
pub struct Coord2D(pub f64, pub f64);

impl Into<Coord<f64>> for Coord2D {
    fn into(self) -> Coord<f64> {
        coord! {x: self.0, y: self.1}
    }
}

impl Into<Coord2D> for Coord<f64> {
    fn into(self) -> Coord2D {
        Coord2D(self.x, self.y)
    }
}

#[wasm_bindgen]
pub fn is_convex(points: Vec<Coord2D>) -> bool {
    LineString::from(points).is_convex()
}
