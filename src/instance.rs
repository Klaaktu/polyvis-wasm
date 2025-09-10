use crate::polygon::PolygonData;
use geo::{Area, IsConvex, LineString};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Instance {
    data: HashMap<u64, PolygonData>,
    counter: u64, // Used for hash map key, not actual count of items!
}

#[wasm_bindgen]
pub enum TextFormat {
    JSON,
    YAML,
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

    // Some fail conditions:
    // Passed id is empty - caller weird
    // filter_map is empty - caller has bad ids
    // union is 0, causes NaN - bad stored data
    pub fn iou(&self, ids: Vec<u64>) -> Result<f64, String> {
        if ids.is_empty() {
            return Err("Called IoU with empty list!".into());
        };

        // peekable needs mut
        let polygons = ids.iter().filter_map(|id| self.data.get(id));
        let mut polygons2 = polygons.clone().peekable();

        if polygons2.peek().is_none() {
            return Err("IDs not found in database! Caller may contain bugs.".into());
        }

        let intersection = PolygonData::unary_intersection(polygons).unsigned_area();
        let union = PolygonData::unary_union(polygons2).unsigned_area();

        if union == 0.0 {
            return Err("Division by 0! Bad polygon area.".into());
        }

        return Ok(intersection / union);
    }

    pub fn serialize(&self, format: TextFormat) -> Result<String, String> {
        return match format {
            TextFormat::JSON => serde_json::to_string(self).map_err(|e| e.to_string()),
            TextFormat::YAML => serde_yaml_ng::to_string(self).map_err(|e| e.to_string()),
        };
    }
}

// Iterator has state, which is why it's usually not shared and some methods take ownership instead of reference.
