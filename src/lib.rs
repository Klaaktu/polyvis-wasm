use geo::{IsConvex, LineString};
use wasm_bindgen::prelude::*;

// Need internal store of polygons, bc the functions use types.
// Seems global variable is rather hard to do in Rust.

#[wasm_bindgen]
pub fn is_convex(points: Vec<[f64; 2]>) -> bool {
    let line: LineString<f64> = points.into();
    return line.is_convex();
}

// TODO: F*ck, the custom type [f64; 2] has no built in conversion from JS
// Need to see how others did it.
// Be back soon!
