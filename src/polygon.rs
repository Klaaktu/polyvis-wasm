use geo::{BooleanOps, LineString, MultiPolygon, Polygon, unary_union};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PolygonData {
    polygon: Polygon,
    color: u32, // Web Color is max 32 bit
    selected: bool,
}

impl PolygonData {
    pub fn new(ext_line: LineString, color: u32, selected: bool) -> Self {
        Self {
            polygon: Polygon::new(ext_line, vec![]),
            color: color,
            selected: selected,
        }
    }

    fn structs_to_polygons<'a>(
        polygon_data: impl IntoIterator<Item = &'a PolygonData>,
    ) -> impl Iterator<Item = &'a Polygon> {
        polygon_data.into_iter().map(|s| &s.polygon)
    }

    pub fn unary_union<'a>(
        polygon_data: impl IntoIterator<Item = &'a PolygonData>,
    ) -> MultiPolygon {
        unary_union(PolygonData::structs_to_polygons(polygon_data))
    }

    // Intersection has no similar unary function
    pub fn unary_intersection<'a>(
        polygon_data: impl IntoIterator<Item = &'a PolygonData>,
    ) -> MultiPolygon {
        let mut polygons = PolygonData::structs_to_polygons(polygon_data);
        let intersection = match polygons.next() {
            None => return MultiPolygon::new(vec![]),
            Some(pointer) => MultiPolygon::new(vec![pointer.clone()]),
        };
        polygons.fold(intersection, |acc, p| acc.intersection(p))
    }
}

// Format u32 color with format!("#{:08X}")
