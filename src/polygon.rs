use geo::{LineString, Polygon};

pub struct PolygonData {
    pub polygon: Polygon,
    pub color: u32, // Web Color is max 32 bit
    pub selected: bool,
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

// Format u32 color with format!("#{:08X}")
