use std::cmp::Ordering::Less;

#[macro_use] extern crate assert_approx_eq;

extern crate itertools;
use itertools::Itertools;

fn cross2d(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

fn area(coord: &[f64]) -> f64 {
    coord.iter()
         .tuples::<(_, _)>()
         .tuple_windows::<(_, _)>()
         .fold(0f64, |sum, ((x1, y1), (x2, y2))| sum + (y1+y2) * (x1-x2)) / 2.
}

// points stores a contiguous array of 2N floats in the format x1, y1, x2, y2, ...
pub fn andrew(pointset: &[f64]) -> Vec<f64> {
    // sort by x coordinates
    let sorted = pointset.iter()
            // .chunks(2)
            .tuples::<(_, _)>()
            .sorted_by(|a, b| {
                if a.0.partial_cmp(b.0).unwrap_or(Less) == Less {
                    Less
                } else {
                    a.1.partial_cmp(b.1).unwrap_or(Less)
                }
            });

    //
    let mut hull = vec![0f64; 2*pointset.len()];
    let mut k = 0;
    for i in sorted.iter().map(|a| (*a.0, *a.1)) {
        if k >= 4 && cross2d((hull[k-4], hull[k-3]), (hull[k-2], hull[k-1]) , i) <= 0f64 {
            k -= 2;
        }
        hull[k] = i.0;
        hull[k+1] = i.1;
        k += 2;
    }
    let t = k+1;
    for i in sorted.iter().rev().map(|a| (*a.0, *a.1)) {
        if k >= t && cross2d((hull[k-4], hull[k-3]), (hull[k-2], hull[k-1]) , i) <= 0f64 {
            k -= 2;
        }
        hull[k] = i.0;
        hull[k+1] = i.1;
        k += 2;
    }
    // -1 because first and last are same
    hull.truncate(k - 1);

    hull
}

#[test]
fn test_hull() {
    let p = vec![
        0., 0.,
        1., 0.,
        0., 1.,
        1., 1.,
    ];

    let expected_area = 1.0;

    assert_approx_eq!(area(&andrew(&p)), expected_area);
}
