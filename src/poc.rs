use geo::{IsConvex, LineString};
use wasm_bindgen::prelude::*;

// Need internal store of polygons, bc the functions use types.
// Seems global variable is rather hard to do in Rust.

#[wasm_bindgen]
pub fn is_convex(points: Vec<f64>) -> bool {
    debug_assert!(
        points.len() % 2 == 0,
        "Malformed! Length of vector is not even."
    );

    let line: LineString<f64> = points
        .chunks_exact(2)
        .map(|s| [s[0], s[1]])
        .collect::<Vec<[f64; 2]>>()
        .into();
    return line.is_convex();
}

// #[wasm_bindgen]
// pub fn iou(polygons: Vec<Vec<[f64; 2]>>) -> f64 {
//     // eh will be very gross
// }

/*
Vec<[f64;2]> cannot be passed from JS to WASM. Solutions:
1. Flatten
2. Export a struct to JS. Cannot export existing struct, need to wrap them as well.
3. Serialize using Serde. Slow and overkill.
Immediatly becomes a problem for multi-polygon input:
Vec<Vec<[f64; 2]>> 2 layers of dynamic length. Has to use Serde.
*/

// #[wasm_bindgen]
// pub struct Point2D(Coord<f64>);
// pub struct Point2D([f64; 2]);

// TODO: Put polygons in WASM shared memory for export support? Serde for JSON?

// Working: .map(|s| s.try_into().expect(""))
// Not working: .map(|s| s.into()) trait not implemented.

// Limitation: github.com/wasm-bindgen/wasm-bindgen/issues/1187
// Result<Self, &'static str>
// return Err("Shape is not convex!");
