use crate::polygon::PolygonData;
use geo::{Area, BooleanOps, IsConvex, LineString, MultiPolygon, unary_union};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Instance {
    data: HashMap<u64, PolygonData>,
    counter: u64, // Used for hash map key, not actual count of items!
}

#[wasm_bindgen]
impl Instance {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Instance {
        return Instance {
            data: HashMap::new(),
            counter: 0,
        };
    }

    pub fn add_polygon(
        &mut self,
        points: Vec<f64>,
        color: u32,
        selected: bool,
    ) -> Result<u64, String> {
        let ext_line = LineString::from(points.as_chunks::<2>().0.to_vec());
        if !ext_line.is_convex() {
            return Err("Shape is not convex!".into());
        }
        let p = PolygonData::new(ext_line, color, selected);
        self.counter += 1;
        self.data.insert(self.counter, p);
        return Ok(self.counter);
    }

    pub fn iou(&self, ids: Vec<u64>) -> Result<f64, String> {
        let polygons = ids
            .iter()
            .filter_map(|id| self.data.get(id))
            .map(|poly_data| &poly_data.polygon);
        let mut iter_clone = polygons.clone();

        // Intersection has no similar unary function
        let mut intersection = match iter_clone.next() {
            None => return Err("Polygon data not found!".into()),
            Some(pointer) => MultiPolygon::new(vec![pointer.clone()]),
        };
        intersection = iter_clone.fold(intersection, |acc, p| acc.intersection(p));
        let intersection = intersection.unsigned_area();

        let union = unary_union(polygons).unsigned_area();

        // Division by 0 in float returns NaN.
        return Ok(intersection / union);
    }
}

// Iterator has state, which is why it's usually not shared and some methods take ownership instead of reference.
