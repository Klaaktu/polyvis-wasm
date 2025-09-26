use crate::Coord2D;
use geo::{BooleanOps, Coord, IsConvex, LineString, MultiPolygon, Polygon};
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

pub fn vec_to_convex_poly(points: Vec<Coord2D>) -> Result<Polygon, String> {
    let ext_line = closed_line(points);
    if !ext_line.is_convex() {
        return Err("Shape is not convex!".into());
    }
    Ok(Polygon::new(ext_line, vec![]))
}

pub fn unary_intersection<'a>(mut polygons: impl Iterator<Item = &'a Polygon>) -> MultiPolygon {
    let intersection = match polygons.next() {
        None => return MultiPolygon::new(vec![]),
        Some(pointer) => MultiPolygon::new(vec![pointer.clone()]),
    };
    polygons.fold(intersection, |acc, p| acc.intersection(p))
}

pub fn rand_convex_poly(n: usize, up_bound: f64) -> Polygon {
    //     if let Ok(p) = crate::algorithms::convex_hull(vertices) {
    //         return p;

    let n = max(n, 3);
    let mut vectors: Vec<Coord> = zip(random_sum_zero(n, up_bound), random_sum_zero(n, up_bound))
        .map(|(a, b)| Coord { x: a, y: b })
        .collect();
    vectors.sort_by(|a, b| a.y.atan2(a.x).total_cmp(&b.y.atan2(b.x)));
    let vertices: Vec<Coord> = vectors
        .iter()
        .scan(Coord::zero(), |&mut mut acc, &v| {
            acc = acc + v;
            Some(acc)
        })
        .collect();
    Polygon::new(LineString::from(vertices), vec![])
}

fn random_sum_zero(n: usize, up_bound: f64) -> Vec<f64> {
    fn scan_iter<'a>(
        slice: &'a [f64],
        min: f64,
        max: &'a f64,
        f: fn(f64, f64) -> f64,
    ) -> impl Iterator<Item = f64> {
        slice
            .iter()
            .chain(once(max))
            .scan(min, move |&mut mut prev, &x| {
                let out = f(prev, x);
                prev = x;
                Some(out)
            })
    }

    debug_assert!(n > 0, "Specified size 0 to random list generator!");
    let mut nums: Vec<f64> = (0..n).map(|_| random_range(0.0..up_bound)).collect();
    nums.sort_unstable_by(f64::total_cmp);
    let min = *nums.first().unwrap();
    let max = nums.last().unwrap();
    let n_pos = random_range(1..n);
    let positive = scan_iter(&nums[..n_pos], min, max, |prev, x| x - prev);
    let negative = scan_iter(&nums[n_pos..], min, max, |prev, x| prev - x);
    let mut res: Vec<f64> = positive.chain(negative).collect();
    res.shuffle(&mut rng());
    res
}

// Random convex polygon:
// [Algorithm](https://stackoverflow.com/questions/6758083/how-to-generate-a-random-convex-polygon#47358689)
// [Referenced implementation](https://github.com/rgeometry/rgeometry/blob/5571d315be90136440c9f08f6581f5c8c13b339d/src/data/polygon/convex.rs#L118)

// TODO
// 1. Why the heck is &mut mut acc, and prev has warning
// 2. Change to return line string instead
// 3. Return of instance problematic
