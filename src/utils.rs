use crate::Coord2D;
use geo::orient::Direction::Default;
use geo::{BooleanOps, Coord, CoordsIter, LineString, MultiPolygon, Orient, Polygon};
use rand::{random_range, rng, seq::SliceRandom};
use std::{
    cmp::max,
    iter::{once, zip},
};

pub fn closed_line(points: Vec<Coord2D>) -> LineString {
    let mut line = LineString::from(points);
    line.close();
    line
}

pub fn new_oriented_poly(line: LineString) -> Polygon {
    Polygon::new(line, vec![]).orient(Default)
}

// pub fn vec_to_convex_poly(points: Vec<Coord2D>) -> Result<Polygon, String> {
//     let ext_line = closed_line(points);
//     if !ext_line.is_convex() {
//         return Err("Shape is not convex!".into());
//     }
//     Ok(Polygon::new(ext_line, vec![]))
// }

pub fn unary_intersection<'a>(mut polygons: impl Iterator<Item = &'a Polygon>) -> MultiPolygon {
    let intersection = match polygons.next() {
        None => return MultiPolygon::new(vec![]),
        Some(pointer) => MultiPolygon::new(vec![pointer.clone()]),
    };
    polygons.fold(intersection, |acc, p| acc.intersection(p))
}

pub fn rand_convex_poly(n: usize, up_bound: f64) -> Polygon {
    let n = max(n, 3);
    let mut vectors: Vec<Coord> = zip(random_sum_zero(n, up_bound), random_sum_zero(n, up_bound))
        .map(|(a, b)| Coord { x: a, y: b })
        .collect();
    // atan2 range is [-pi,pi], sort in reverse order so the polygon is upright
    vectors.sort_by(|a, b| b.y.atan2(b.x).total_cmp(&a.y.atan2(a.x)));
    let line = vectors
        .iter()
        .scan(Coord::zero(), |acc, &v| {
            *acc = *acc + v;
            Some(*acc)
        })
        .collect();
    new_oriented_poly(line)
}

fn random_sum_zero(n: usize, up_bound: f64) -> Vec<f64> {
    fn scan_iter<'a>(
        slice: &'a [f64],
        min: f64,
        max: &'a f64,
        f: fn(f64, f64) -> f64,
    ) -> impl Iterator<Item = f64> {
        slice.iter().chain(once(max)).scan(min, move |prev, &x| {
            let out = f(*prev, x);
            *prev = x;
            Some(out)
        })
    }

    debug_assert!(n > 0, "Specified size 0 to random list generator!");
    let mut nums: Vec<f64> = (0..n).map(|_| random_range(0.0..up_bound)).collect();
    nums.sort_unstable_by(f64::total_cmp);
    let min = *nums.first().unwrap();
    let max = nums.last().unwrap();
    let n_pos = random_range(1..n);
    let positive = scan_iter(&nums[1..n_pos], min, max, |prev, x| x - prev);
    let negative = scan_iter(&nums[n_pos..n - 1], min, max, |prev, x| prev - x);
    let mut res: Vec<f64> = positive.chain(negative).collect();
    res.shuffle(&mut rng());
    res
}

// Not returning the last vertex of the polygon, which is the same as the first.
// No better way, right? Cannot skip last in iter due to nature of iter, cannot return slice due to ownership.
pub fn poly_to_js_coord(p: &Polygon) -> Vec<Coord2D> {
    let mut res: Vec<Coord2D> = p.exterior_coords_iter().map(|c| c.into()).collect();
    res.pop();
    res
}

// Random convex polygon:
// [Algorithm](https://stackoverflow.com/questions/6758083/how-to-generate-a-random-convex-polygon#47358689)
// [Referenced implementation](https://github.com/rgeometry/rgeometry/blob/5571d315be90136440c9f08f6581f5c8c13b339d/src/data/polygon/convex.rs#L118)
