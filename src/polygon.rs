use geo::{
    BooleanOps, CoordinatePosition, LineString, MultiPolygon, Polygon,
    coordinate_position::CoordPos,
};
use serde::{Deserialize, Serialize};

use crate::Coord2D;

#[derive(Serialize, Deserialize)]
pub struct PolygonData {
    polygon: Polygon,
    color: u32, // Web Color is max 32 bit
    id: u32,
}

impl PolygonData {
    pub fn new(ext_line: LineString, color: u32, id: u32) -> Self {
        Self {
            polygon: Polygon::new(ext_line, vec![]),
            color: color,
            id: id,
        }
    }

    fn structs_to_polygons<'a>(
        polygon_data: impl Iterator<Item = &'a PolygonData>,
    ) -> impl Iterator<Item = &'a Polygon> {
        polygon_data.into_iter().map(|s| &s.polygon)
    }

    pub fn unary_union<'a>(polygon_data: impl Iterator<Item = &'a PolygonData>) -> MultiPolygon {
        geo::unary_union(PolygonData::structs_to_polygons(polygon_data))
    }

    // Intersection has no similar unary function
    pub fn unary_intersection<'a>(
        polygon_data: impl Iterator<Item = &'a PolygonData>,
    ) -> MultiPolygon {
        let mut polygons = PolygonData::structs_to_polygons(polygon_data);
        let intersection = match polygons.next() {
            None => return MultiPolygon::new(vec![]),
            Some(pointer) => MultiPolygon::new(vec![pointer.clone()]),
        };
        polygons.fold(intersection, |acc, p| acc.intersection(p))
    }

    pub fn coord_in_polygon(&self, c: Coord2D) -> bool {
        self.polygon.coordinate_position(&c.into()) != CoordPos::Outside
    }
}

// Format u32 color with format!("#{:08X}")
