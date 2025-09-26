use crate::{
    api::Coord2D,
    utils::{rand_convex_poly, unary_intersection, vec_to_convex_poly},
};
use geo::{Area, Coord, CoordinatePosition, Polygon, coordinate_position::CoordPos, unary_union};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, vec};
use wasm_bindgen::prelude::*;

static S1: &str = "IDs not found in database! Caller may contain bugs.";

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Instance {
    data: HashMap<u32, Polygon>,
    counter: u32, // Used for hash map key, not actual count of items!
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
        Instance {
            data: HashMap::new(),
            counter: 0,
        }
    }

    pub fn add_polygon(&mut self, points: Vec<Coord2D>) -> Result<u32, String> {
        let p = vec_to_convex_poly(points)?;
        self.counter += 1;
        self.data.insert(self.counter, p);
        Ok(self.counter)
    }

    pub fn mod_polygon(&mut self, id: u32, points: Vec<Coord2D>) -> Result<(), String> {
        let p = vec_to_convex_poly(points)?;
        self.data.insert(id, p);
        Ok(())
    }

    pub fn del_polygon(&mut self, id: u32) -> Result<(), String> {
        self.data.remove(&id).ok_or(S1.into()).map(|_| ())
    }

    // Some fail conditions:
    // Passed id is empty - caller weird
    // filter_map is empty - caller has bad ids
    // union is 0, causes NaN - bad stored data
    pub fn iou(&self, ids: Vec<u32>) -> Result<f64, String> {
        // peekable needs mut
        let polygons = self.ids_to_polygons(&ids)?;
        let mut polygons2 = polygons.clone().peekable();

        if polygons2.peek().is_none() {
            return Err(S1.into());
        }

        let intersection = unary_intersection(polygons).unsigned_area();
        let union = unary_union(polygons2).unsigned_area();

        if union == 0.0 {
            return Err("Division by 0! Bad polygon area.".into());
        }

        Ok(intersection / union)
    }

    pub fn polygons_under_coord(&self, c: Coord2D) -> Option<u32> {
        let geo_coord: Coord = c.into();
        self.data
            .iter()
            .filter(|(_, poly)| poly.coordinate_position(&geo_coord) != CoordPos::Outside)
            .map(|(&id, _)| id)
            .max()
    }

    // Not to be used in IoU
    // This assumes the intersection is always 1 polygon
    pub fn intersection(&self, ids: Vec<u32>) -> Result<Vec<Coord2D>, String> {
        let polygon_data = self.ids_to_polygons(&ids)?;
        let i = match unary_intersection(polygon_data).iter().next() {
            Some(p) => p.exterior().coords().map(|c| (*c).into()).collect(),
            None => vec![],
        };
        Ok(i)
    }

    pub fn rand_convex_poly(&self, n: usize, up_bound: f64) -> (u32, Vec<Coord2D>) {
        let p = rand_convex_poly(n, up_bound);
        self.counter += 1;
        self.data.insert(self.counter, p);
        (self.counter, p)
    }

    pub fn serialize(&self, format: TextFormat) -> Result<String, String> {
        match format {
            TextFormat::JSON => serde_json::to_string(self).map_err(|e| e.to_string()),
            TextFormat::YAML => serde_yaml_ng::to_string(self).map_err(|e| e.to_string()),
        }
    }

    fn ids_to_polygons(
        &self,
        ids: &Vec<u32>,
    ) -> Result<impl Iterator<Item = &Polygon> + Clone, String> {
        if ids.is_empty() {
            return Err("Empty list of IDs!".into());
        };
        Ok(ids.iter().filter_map(|id| self.data.get(id)))
    }
}

// Iterator has state, which is why it's usually not shared and some methods take ownership instead of reference.
