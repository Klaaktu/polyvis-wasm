use crate::{Coord2D, polygon::PolygonData};
use geo::{Area, IsConvex, LineString};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, vec};
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

    pub fn add_polygon(&mut self, points: Vec<f64>, color: u32) -> Result<u64, String> {
        let ext_line = LineString::from(points.as_chunks::<2>().0.to_vec());
        if !ext_line.is_convex() {
            return Err("Shape is not convex!".into());
        }
        let p = PolygonData::new(ext_line, color);
        self.counter += 1;
        self.data.insert(self.counter, p);
        return Ok(self.counter);
    }

    // Some fail conditions:
    // Passed id is empty - caller weird
    // filter_map is empty - caller has bad ids
    // union is 0, causes NaN - bad stored data
    pub fn iou(&self, ids: Vec<u64>) -> Result<f64, String> {
        // peekable needs mut
        let polygons = self.ids_to_polygons(&ids)?;
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

    // Not to be used in IoU
    // This assumes the intersection is always 1 polygon
    pub fn intersection(&self, ids: Vec<u64>) -> Result<Vec<Coord2D>, String> {
        let polygon_data = self.ids_to_polygons(&ids)?;
        let i = match PolygonData::unary_intersection(polygon_data).iter().next() {
            Some(p) => p.exterior().coords().map(|c| (*c).into()).collect(),
            None => vec![],
        };
        Ok(i)
    }

    pub fn serialize(&self, format: TextFormat) -> Result<String, String> {
        return match format {
            TextFormat::JSON => serde_json::to_string(self).map_err(|e| e.to_string()),
            TextFormat::YAML => serde_yaml_ng::to_string(self).map_err(|e| e.to_string()),
        };
    }

    fn ids_to_polygons(
        &self,
        ids: &Vec<u64>,
    ) -> Result<impl Iterator<Item = &PolygonData> + Clone, String> {
        if ids.is_empty() {
            return Err("Empty list of IDs!".into());
        };
        Ok(ids.iter().filter_map(|id| self.data.get(id)))
    }
}

// Iterator has state, which is why it's usually not shared and some methods take ownership instead of reference.
