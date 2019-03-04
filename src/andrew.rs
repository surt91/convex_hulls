use itertools::Itertools;
use std::cmp::Ordering::{Less, Equal};

use primitives::cross2d;

// points stores a contiguous array of 2N floats in the format x1, y1, x2, y2, ...
#[cfg(not(feature = "visual"))]
pub fn andrew(pointset: &[f64]) -> Vec<f64> {
    if pointset.len() < 3*2 {
        return pointset.to_vec()
    }

    // sort by x coordinates
    let mut sorted: Vec<(f64, f64)> = pointset.iter()
        .cloned()
        .tuples::<(_, _)>()
        .collect();

    // sort by x and on tie by y
    sorted.sort_unstable_by(|a, b| {
            let tmp = a.0.partial_cmp(&b.0).unwrap_or(Less);
            if tmp != Equal {
                tmp
            } else {
                a.1.partial_cmp(&b.1).unwrap_or(Less)
            }
        });

    let mut hull = Vec::new();
    let mut k = 0;
    for i in sorted.iter() {
        while k >= 4 && cross2d((hull[k-4], hull[k-3]), (hull[k-2], hull[k-1]), *i) <= 0f64 {
            hull.pop();
            hull.pop();
            k -= 2;
        }
        hull.push(i.0);
        hull.push(i.1);
        k += 2;
    }
    let t = k+2;
    for i in sorted.iter().rev() {
        while k >= t && cross2d((hull[k-4], hull[k-3]), (hull[k-2], hull[k-1]), *i) <= 0f64 {
            hull.pop();
            hull.pop();
            k -= 2;
        }
        hull.push(i.0);
        hull.push(i.1);
        k += 2;
    }
    // -2 because first and last are same
    hull.pop();
    hull.pop();

    hull
}

#[cfg(feature = "visual")]
use visualization::SVG;

// points stores a contiguous array of 2N floats in the format x1, y1, x2, y2, ...
#[cfg(feature = "visual")]
pub fn andrew(pointset: &[f64]) -> Vec<f64> {
    if pointset.len() < 3*2 {
        return pointset.to_vec()
    }

    // sort by x coordinates
    let mut sorted: Vec<(f64, f64)> = pointset.iter()
        .cloned()
        .tuples::<(_, _)>()
        .collect();

    // sort by x and on tie by y
    sorted.sort_unstable_by(|a, b| {
            let tmp = a.0.partial_cmp(&b.0).unwrap_or(Less);
            if tmp != Equal {
                tmp
            } else {
                a.1.partial_cmp(&b.1).unwrap_or(Less)
            }
        });

    let mut g = 0;

    let mut hull = Vec::new();
    let mut k = 0;
    for i in sorted.iter() {
        while k >= 4 && cross2d((hull[k-4], hull[k-3]), (hull[k-2], hull[k-1]), *i) <= 0f64 {
            hull.pop();
            hull.pop();
            k -= 2;
        }
        hull.push(i.0);
        hull.push(i.1);
        k += 2;

        g += 1;
        let filename = format!("img/andrew_{:04}.svg", g);
        let mut s = SVG::new();
        s.points(pointset, "grey");
        s.points(&hull, "black");
        s.lines(&hull, "black");
        s.points(&[i.0, i.1], "red");
        s.save(&filename);
    }
    let t = k+2;
    for i in sorted.iter().rev() {
        while k >= t && cross2d((hull[k-4], hull[k-3]), (hull[k-2], hull[k-1]), *i) <= 0f64 {
            hull.pop();
            hull.pop();
            k -= 2;
        }
        hull.push(i.0);
        hull.push(i.1);
        k += 2;

        g += 1;
        let filename = format!("img/andrew_{:04}.svg", g);
        let mut s = SVG::new();
        s.points(pointset, "grey");
        s.points(&hull, "black");
        s.lines(&hull, "black");
        s.points(&[i.0, i.1], "red");
        s.save(&filename);
    }
    // -2 because first and last are same
    hull.pop();
    hull.pop();

    hull
}
