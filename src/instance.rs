use crate::{
    api::Coord2D,
    utils::{rand_convex_verts, unary_intersection},
};
use geo::{
    Area, BooleanOps, Coord, CoordinatePosition, Intersects, LineString, Orient, Polygon,
    coordinate_position::CoordPos, orient::Direction::Default, unary_union,
};
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

// pub field for wasm-bindgen must implement Copy, or custom getter (e.g. with clone)
#[wasm_bindgen]
pub struct PolyAId(pub u32, #[wasm_bindgen(getter_with_clone)] pub Vec<Coord2D>);

#[wasm_bindgen]
impl Instance {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Instance {
        Instance {
            data: HashMap::new(),
            counter: 0,
        }
    }

    pub fn add_polygon(&mut self, points: Vec<Coord2D>) -> u32 {
        self.counter += 1;
        self.data
            .insert(self.counter, new_oriented_poly(points.into()));
        self.counter
    }

    pub fn mod_polygon(&mut self, id: u32, points: Vec<Coord2D>) -> () {
        self.data.insert(id, new_oriented_poly(points.into()));
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

    pub fn rand_convex_poly(&mut self, n: usize, up_bound: f64) -> PolyAId {
        let verts = rand_convex_verts(n, up_bound);
        let res: Vec<Coord2D> = verts.iter().map(|&e| e.into()).collect();
        self.counter += 1;
        self.data
            .insert(self.counter, new_oriented_poly(verts.into()));
        PolyAId(self.counter, res)
    }

    pub fn serialize(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }

    /// Import from a text containing a vector of polygons,
    /// where each polygon is a vector of coords, and each cord is an array of (two) f64.
    pub fn import_list(&mut self, text: &str) -> Result<(), String> {
        let vec: Vec<Vec<[f64; 2]>> =
            serde_json::from_str(text).map_err(|_| "Parsing input failed!")?;
        let polygons = vec.iter().map(|verts| {
            new_oriented_poly(verts.iter().map(|c| Coord { x: c[0], y: c[1] }).collect())
        });
        let n: u32 = vec
            .len()
            .try_into()
            .map_err(|_| "Length may be too large!")?;
        let kvs = (self.counter + 1..self.counter + 1 + n).zip(polygons);
        self.data.extend(kvs);
        self.counter += n;
        Ok(())
    }

    pub fn dump_to_js(&self) -> Vec<PolyAId> {
        self.data
            .iter()
            .map(|(id, p)| PolyAId(*id, p.exterior().coords().map(|c| (*c).into()).collect()))
            .collect()
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

    // Currently not exported because this feature is not required and I don't want another wrapper struct.
    fn all_pair_iou(&self) -> Vec<(u32, Vec<(u32, f64)>)> {
        let kvs: Vec<(&u32, &Polygon)> = self.data.iter().collect();
        let n = kvs.len();
        let mut res = vec![];
        for i in 0..n {
            let mut intersections = vec![];
            for j in i + 1..n {
                if kvs[i].1.intersects(kvs[j].1) {
                    intersections.push((*kvs[j].0, iou_simple(kvs[i].1, kvs[j].1)))
                }
            }
            if !intersections.is_empty() {
                res.push((*kvs[i].0, intersections))
            };
        }
        res
    }
}

fn iou_simple(a: &Polygon, b: &Polygon) -> f64 {
    let inter = a.intersection(b).unsigned_area();
    let union = a.union(b).unsigned_area();
    debug_assert!(union != 0.0, "Union is 0. Division by 0!");
    inter / union
}

fn new_oriented_poly(line: LineString) -> Polygon {
    Polygon::new(line, vec![]).orient(Default)
}
// Iterator has state, which is why it's usually not shared and some methods take ownership instead of reference.
