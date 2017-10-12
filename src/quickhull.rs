use itertools::Itertools;

use primitives::cross2d;

pub fn quickhull(pointset: &[f64]) -> Vec<f64> {
    if pointset.len() < 3*2 {
        return pointset.to_vec()
    }

    let start = (pointset[0], pointset[1]);
    let (min, max) = pointset.iter()
        .tuples::<(_, _)>()
        .fold((start, start), |(min, max), (&x, &y)| {
            let min = if x < min.0 { (x, y) } else { min };
            let max = if x > max.0 { (x, y) } else { max };
            (min, max)
        });

    let mut hull: Vec<f64> = Vec::new();

    qh_recursion(pointset, min, max, &mut hull);
    qh_recursion(pointset, max, min, &mut hull);

    hull
}

fn qh_recursion(pointset: &[f64], a: (f64, f64), b: (f64, f64), out: &mut Vec<f64>) {
    // find left and farthest away point q
    let left_of: Vec<f64> = pointset.iter()
        .cloned()
        .tuples::<(_, _)>()
        .filter(|&i| cross2d(a, i, b) > 0f64)
        .fold(Vec::new(), |mut acc, p| { acc.push(p.0); acc.push(p.1); acc });

    // if there is none: add b to out and return
    if left_of.len() == 0 {
        let n = out.len();
        // ensure no collinear points
        if n < 4 || cross2d((out[n-4], out[n-3]), (out[n-2], out[n-1]), b) > 0f64 {
            out.push(b.0);
            out.push(b.1);
        } else {
            out[n-2] = b.0;
            out[n-1] = b.1;
        }
    } else {
        // else recurse with the edge (a, q) and (q, b)
        let q = left_of.iter()
            .cloned()
            .tuples::<(_, _)>()
            .fold(b, |farthest: (f64, f64), i: (f64, f64)| if cross2d(a, farthest, b) > cross2d(a, i, b) {farthest} else {i});

        qh_recursion(&left_of, a, q, out);
        qh_recursion(&left_of, q, b, out);
    }
}
