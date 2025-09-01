use geo::{Area, BooleanOps, IsConvex, LineString, MultiPolygon, Polygon, unary_union};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct PolygonData {
    polygon: Polygon,
    color: u32,
    selected: bool,
}

impl PolygonData {
    pub fn new(ext_line: LineString, color: u32, selected: bool) -> Self {
        return Self {
            polygon: Polygon::new(ext_line, vec![]),
            color: color,
            selected: selected,
        };
    }
}

// I need global / static hash map. Alternative is once_cell crate.
#[wasm_bindgen]
pub fn initialize() -> Instance {
    return Instance::new();
}

#[wasm_bindgen]
struct Instance {
    data: HashMap<u32, PolygonData>,
}

#[wasm_bindgen]
impl Instance {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Instance {
        return Instance {
            data: HashMap::new(),
        };
    }

    pub fn add_polygon(
        &mut self,
        id: u32,
        points: Vec<f64>,
        color: u32,
        selected: bool,
    ) -> Result<(), String> {
        let ext_line = LineString::from(points.as_chunks::<2>().0.to_vec());
        if !ext_line.is_convex() {
            return Err(String::from("Shape is not convex!"));
        }
        let p = PolygonData::new(ext_line, color, selected);
        self.data.insert(id, p);
        return Ok(());
    }

    pub fn iou(&self, ids: Vec<u32>) -> f64 {
        let polygons = ids
            .iter()
            .filter_map(|id| self.data.get(id))
            .map(|poly_data| &poly_data.polygon);

        let intersection: f64 = polygons
            // Iterator has state, which is why it's usually not shared. map() takes ownership instead of reference.
            .clone()
            .cloned()
            .map(|p| MultiPolygon::new(vec![p]))
            .reduce(|acc, p| acc.intersection(&p))
            .map(|p| p.unsigned_area())
            .unwrap_or(0.0);

        let union = unary_union(polygons).unsigned_area();

        // Division by 0 in float returns NaN.
        return intersection / union;
    }
}

// Format u32 color with format!("#{:08X}")
// TODO: Can JS handle NaN?
