use itertools::Itertools;

use primitives::{cross2d, dist2};

#[cfg(not(feature = "visual"))]
pub fn jarvis(pointset: &[f64]) -> Vec<f64> {
    if pointset.len() < 3*2 {
        return pointset.to_vec()
    }

    let start = (pointset[0], pointset[1]);
    let min = pointset.iter()
        .tuples::<(_, _)>()
        .fold(start, |min, (&x, &y)| {
            if x < min.0 { (x, y) } else { min }
        });

    let mut hull = Vec::new();
    hull.push(min.0);
    hull.push(min.1);

    let mut p = pointset.iter()
        .cloned()
        .tuples::<(_, _)>()
        .filter(|&i| i != min)
        .nth(0)
        .unwrap();

    loop {
        for i in pointset.iter()
            .cloned()
            .tuples::<(_, _)>()
        {
            let a = (hull[hull.len()-2], hull[hull.len()-1]);
            let orientation = cross2d(a, i, p);
            if orientation > 0f64 {
                p = i;
            } else if orientation == 0f64 {
                // take the one furthest away, to avoid collinear points
                if dist2(a, p) < dist2(a, i) {
                    p = i;
                }
            }
        }
        if p == min {
            break;
        }
        hull.push(p.0);
        hull.push(p.1);
    }

    hull
}

#[cfg(feature = "visual")]
use visualization::SVG;

#[cfg(feature = "visual")]
pub fn jarvis(pointset: &[f64]) -> Vec<f64> {
    let start = (pointset[0], pointset[1]);
    let min = pointset.iter()
        .tuples::<(_, _)>()
        .fold(start, |min, (&x, &y)| {
            if x < min.0 { (x, y) } else { min }
        });

    let mut hull = Vec::new();
    hull.push(min.0);
    hull.push(min.1);

    let mut p = pointset.iter()
        .cloned()
        .tuples::<(_, _)>()
        .filter(|&i| i != min)
        .nth(0)
        .unwrap();

    let mut k = 0;
    loop {
        for i in pointset.iter()
            .cloned()
            .tuples::<(_, _)>()
        {
            let a = (hull[hull.len()-2], hull[hull.len()-1]);
            let orientation = cross2d(a, i, p);

            if orientation >= 0f64 {
                k += 1;
                let filename = format!("img/jarvis_{:04}.svg", k);
                let mut s = SVG::new();
                s.points(pointset, "grey");
                s.points(&hull, "black");
                s.lines(&hull, "black");
                s.lines(&[p.0, p.1, a.0, a.1], "green");
                s.points(&[p.0, p.1], "green");
                s.points(&[i.0, i.1], "red");
                s.save(&filename);
            }

            if orientation > 0f64 {
                p = i;
            } else if orientation == 0f64 {
                // take the one furthest away, to avoid collinear points
                if dist2(a, p) < dist2(a, i) {
                    p = i;
                }
            }
        }
        if p == min {
            break;
        }
        hull.push(p.0);
        hull.push(p.1);

    }

    hull
}
