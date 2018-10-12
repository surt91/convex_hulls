use std::iter;
use itertools::Itertools;

use primitives::{cross2d, tangent, dist2};

use super::andrew::andrew;

// points stores a contiguous array of 2N floats in the format x1, y1, x2, y2, ...
#[cfg(not(visual))]
pub fn chan(pointset: &[f64]) -> Vec<f64> {
    if pointset.len() < 3*2 {
        return pointset.to_vec()
    }

    let mut m = 3;

    // starting conditions for Jarvis part can be precalculated
    let start = (pointset[0], pointset[1]);
    let min = pointset.iter()
        .tuples::<(_, _)>()
        .fold(start, |min, (&x, &y)| {
            if x < min.0 { (x, y) } else { min }
        });

    loop {
        // split the pointset into k = n/m subsets of size <= m
        let subsets: Vec<_> = pointset.chunks(m*2)
            .collect();
        // calculate the subhulls with andrews
        // also make them cyclic and partition into tuples
        let mut subhulls: Vec<_> = subsets.iter()
            .map(|s| {
                let hull: Vec<_> = andrew(s);
                let first = (hull[0], hull[1]);
                hull.iter()
                    .cloned()
                    .tuples::<(_, _)>()
                    .chain(iter::once(first))
                    .collect()
            })
            .collect();

        let mut hull = Vec::new();
        hull.push(min.0);
        hull.push(min.1);

        let mut q = min;

        for _ in 0..m {
            let mut all_t: Vec<f64> = Vec::new();
            for s in &mut subhulls {
                let a = (hull[hull.len()-2], hull[hull.len()-1]);
                let t = tangent(a, s);

                all_t.push(t.0);
                all_t.push(t.1);

                let orientation = cross2d(a, t, q);
                if orientation > 0f64 {
                    q = t;
                } else if orientation == 0f64 {
                    // take the one furthest away, to avoid collinear points
                    if dist2(a, q) < dist2(a, t) {
                        q = t;
                    }
                }
            }

            if q == min {
                return hull;
            }
            hull.push(q.0);
            hull.push(q.1);
        }
        m *= m;
        hull.clear();
    }
}

#[cfg(visual)]
use visualization::SVG;

// points stores a contiguous array of 2N floats in the format x1, y1, x2, y2, ...
#[cfg(visual)]
pub fn chan(pointset: &[f64]) -> Vec<f64> {
    if pointset.len() < 3*2 {
        return pointset.to_vec()
    }

    let mut m = 3;

    // starting conditions for Jarvis part can be precalculated
    let start = (pointset[0], pointset[1]);
    let min = pointset.iter()
        .tuples::<(_, _)>()
        .fold(start, |min, (&x, &y)| {
            if x < min.0 { (x, y) } else { min }
        });

    let mut g = 0;

    loop {
        // split the pointset into k = n/m subsets of size <= m
        let subsets: Vec<_> = pointset.chunks(m*2)
            .collect();
        // calculate the subhulls with andrews
        // also make them cyclic and partition into tuples
        let mut subhulls: Vec<Vec<_>> = subsets.iter()
            .map(|s| {
                let hull: Vec<_> = andrew(s);
                let first = (hull[0], hull[1]);
                hull.iter()
                    .cloned()
                    .tuples::<(_, _)>()
                    .chain(iter::once(first))
                    .collect()
            })
            .collect();

        let filename = format!("img/chan_{:04}.svg", g);
        let mut s = SVG::new();
        s.points(pointset, "grey");
        for h in subhulls.clone() {
            let h: Vec<f64> = h.iter().flat_map(|tup| iter::once(tup.0).chain(iter::once(tup.1))).collect();
            s.polygon(&h, "grey");
        }
        s.save(&filename);

        g += 1;

        let mut hull = Vec::new();
        hull.push(min.0);
        hull.push(min.1);

        let mut q = min;

        for _ in 0..m {
            let mut all_t: Vec<f64> = Vec::new();
            for s in &mut subhulls {
                let a = (hull[hull.len()-2], hull[hull.len()-1]);
                let t = tangent(a, s);

                all_t.push(t.0);
                all_t.push(t.1);

                let orientation = cross2d(a, t, q);
                if orientation > 0f64 {
                    q = t;
                } else if orientation == 0f64 {
                    // take the one furthest away, to avoid collinear points
                    if dist2(a, q) < dist2(a, t) {
                        q = t;
                    }
                }
            }

            let filename = format!("img/chan_{:04}.svg", g);
            let mut s = SVG::new();
            s.points(pointset, "grey");
            for h in subhulls.clone() {
                let h: Vec<f64> = h.iter().flat_map(|tup| iter::once(tup.0).chain(iter::once(tup.1))).collect();
                s.polygon(&h, "grey");
            }
            s.lines(&hull, "black");
            for (&t1, &t2) in all_t.iter().clone().tuples::<(_, _)>() {
                s.dashed_lines(&[hull[hull.len()-2], hull[hull.len()-1], t1, t2], "green");
            }
            s.points(&all_t, "green");
            s.points(&hull, "black");
            s.points(&[q.0, q.1], "red");
            s.save(&filename);
            g += 1;

            if q == min {
                let filename = format!("img/chan_{:04}.svg", g);
                let mut s = SVG::new();
                s.points(pointset, "grey");
                for h in subhulls.clone() {
                    let h: Vec<f64> = h.iter().flat_map(|tup| iter::once(tup.0).chain(iter::once(tup.1))).collect();
                    s.polygon(&h, "grey");
                }
                s.polygon(&hull, "black");
                s.points(&hull, "black");
                s.save(&filename);

                return hull;
            }
            hull.push(q.0);
            hull.push(q.1);
        }
        m *= m;
        hull.clear();
    }
}
