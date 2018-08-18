use std::iter;
use itertools::Itertools;

pub fn side(p1: (f64, f64), p2: (f64, f64), p: (f64, f64)) -> f64 {
    return (p2.1 - p1.1)*(p.0 - p1.0) + (-p2.0 + p1.0)*(p.1 - p1.1);
}

pub fn point_in_octagon(octagon: [(f64, f64); 8], p: (f64, f64)) -> bool {
    // compare opposite sites first
       side(octagon[0], octagon[1], p) > 0f64
    && side(octagon[2], octagon[3], p) > 0f64
    && side(octagon[4], octagon[5], p) > 0f64
    && side(octagon[6], octagon[7], p) > 0f64
    && side(octagon[1], octagon[2], p) > 0f64
    && side(octagon[3], octagon[4], p) > 0f64
    && side(octagon[5], octagon[6], p) > 0f64
    && side(octagon[7], octagon[0], p) > 0f64
}

pub fn dist2(a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - b.0)*(a.0 - b.0) + (a.1 - b.1)*(a.1 - b.1)
}

pub fn cross2d(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

pub fn area(coord: &[f64]) -> f64 {
    coord.iter()
         .chain(coord.iter().take(2)) // append the first point, to close the loop
         .tuples::<(_, _)>()
         .tuple_windows::<(_, _)>()
         .fold(0f64, |sum, ((x1, y1), (x2, y2))| sum + (y1+y2) * (x1-x2)) / 2.
}


fn orientation(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> i8 {
    match -cross2d(o, a, b) {
        x if x > 0. => -1,  // CW
        x if x < 0. => 1,   // CCW
        _ => 0,             // Collinear
    }
}

// cmp: https://github.com/ypranay/Convex-Hull/blob/master/ChansAlgorithmForConvexHull.cpp
pub fn tangent(p: (f64, f64), poly: &Vec<f64>) -> (f64, f64) {
    // search for the tangent through `p` of the polygon `poly`
    // use a clever binary search
    // all points q before the tangent t are ptq oriented ccw and after cw

    if poly.len() == 2 {
        return (poly[0], poly[1]);
    }

    // do not find yourself
    let mut poly = poly.iter()
        .cloned()
        .tuples::<(f64, f64)>()
        .filter(|&q| q != p);

    // repeat the first point in the end
    let first = poly.next().unwrap();
    let poly: Vec<(f64, f64)> = iter::once(first)
        .chain(poly)
        .chain(iter::once(first))
        .collect();

    let n = poly.len();
    let mut l = 0;
    let mut r = n;
    let mut l_before = orientation(p, poly[0], poly[n-1]);
    let mut l_after = orientation(p, poly[0], poly[(l + 1) % n]);
    while l < r {
        let c = (l + r)/2;
        let c_before = orientation(p, poly[c], poly[(c - 1) % n]);
        let c_after = orientation(p, poly[c], poly[(c + 1) % n]);
        let c_side = orientation(p, poly[l], poly[c]);
        if c_before >= 0 && c_after >= 0 {
            return poly[c];
        } else if ((c_side > 0) && (l_after < 0 || l_before == l_after)) || (c_side < 0 && c_before < 0) {
            r = c;
        } else {
            l = c + 1 ;
        }
        l_before = -c_after;
        l_after = orientation(p, poly[l], poly[(l + 1) % n]);
    }
    return poly[l];
}
