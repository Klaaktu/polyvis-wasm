use crate::{instance::Instance, utils::closed_line};
use geo::{Coord, IsConvex, coord};
use wasm_bindgen::prelude::*;

// Call this in JS main(). This is like main() for lib.
// I need global / static hash map. Alternative is LazyLock.
#[wasm_bindgen]
pub fn new_session() -> Instance {
    Instance::new()
}

#[wasm_bindgen]
pub fn deserialize_session(text: &str) -> Result<Instance, String> {
    serde_json::from_str(text).map_err(|_| "Parsing input failed!".into())
}

// Wrapper struct for geo::Coord. wasm_bindgen cannot export crate items.
// wasm_bindgen can only export struct not tuple or array
// Coord2D is made copy-able to make Vec<Coord2D> clone-able. Copy requires Clone so derive both.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Coord2D(pub f64, pub f64);

// Provide a constructor, otherwise constructor is private in JS.
#[wasm_bindgen]
impl Coord2D {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Coord2D {
        Coord2D(x, y)
    }
}

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
    closed_line(points).is_convex()
}

// Only closed LineString can be convex. https://docs.rs/geo/latest/geo/algorithm/is_convex/trait.IsConvex.html
