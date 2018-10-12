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

fn cw(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> bool {
    return cross2d(o, a, b) > 0.;
}
fn ccw(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> bool {
    return cross2d(o, a, b) < 0.;
}

// cmp: https://github.com/felipesfaria/ch_chan/blob/master/ch_chan/ch_chan.cpp
pub fn tangent(p: (f64, f64), poly: &mut Vec<(f64, f64)>) -> (f64, f64) {
    // search for the tangent through `p` of the polygon `poly`
    // use a clever binary search
    // all points q before the tangent t are ptq oriented ccw and after cw

    // special case of single points
    if poly.len() == 2 {
        return poly[0];
    }

    // we have a problem if p is the first point of the hull for some reason
    // in that case I just reorder in O(N), which hopefully happens seldom
    // enough to not influence the runtime too bad
    if poly[0] == p {
        let n = poly.len();
        let first = poly[1];
        for i in 0..(n-1) {
            poly[i] = poly[i+1];
        }
        poly[n-1] = first;
    }

    let n = poly.len()-1;
    let mut a = 0; // lower
    let mut b = n; // upper
    let mut c;     // mid
    let mut ccw_a;
    let mut cw_c;

    // rightmost tangent = maximum for the isLeft() ordering
    // test if poly[0] is a local maximum
    if ccw(p, poly[1], poly[0]) && !cw(p, poly[n-1], poly[0]) {
        return poly[0];
    }

    loop {
        c = (a + b) / 2;

        // if we encounter our selves, the next point is a tangent (due to convexivity)
        if poly[c] == p {
            return poly[c+1];
        }

        cw_c = ccw(p, poly[c+1], poly[c]);
        // is c the tangent?
        if cw_c && !cw(p, poly[c-1], poly[c]) {
            return poly[c];
        }

        // continue with the binary search
        ccw_a = cw(p, poly[a+1], poly[a]);
        if ccw_a {
            if cw_c {
                b = c;
            } else {
                if cw(p, poly[a], poly[c]) {
                    b = c;
                } else {
                    a = c;
                }
            }
        }
        else {
            if !cw_c {
                a = c;
            } else {
                if ccw(p, poly[a], poly[c]) {
                    b = c;
                } else {
                    a = c;
                }
            }
        }
    }
}
