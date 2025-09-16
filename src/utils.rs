use crate::Coord2D;
use geo::LineString;

pub fn closed_line(points: Vec<Coord2D>) -> LineString {
    let mut line = LineString::from(points);
    line.close();
    line
}
