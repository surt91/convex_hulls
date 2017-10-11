#![feature(test)]
extern crate test;

#[macro_use]
extern crate assert_approx_eq;

use std::cmp::Ordering::Less;

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
        while k >= 4 && cross2d((hull[k-4], hull[k-3]), (hull[k-2], hull[k-1]) , i) <= 0f64 {
            k -= 2;
        }
        hull[k] = i.0;
        hull[k+1] = i.1;
        k += 2;
    }
    let t = k+2;
    for i in sorted.iter().rev().map(|a| (*a.0, *a.1)) {
        while k >= t && cross2d((hull[k-4], hull[k-3]), (hull[k-2], hull[k-1]) , i) <= 0f64 {
            k -= 2;
        }
        hull[k] = i.0;
        hull[k+1] = i.1;
        k += 2;
    }
    // -2 because first and last are same
    hull.truncate(k - 2);

    hull
}

pub fn quickhull(pointset: &[f64]) -> Vec<f64> {
    let (min_x, max_x) = pointset.iter()
        .tuples::<(_, _)>()
        .fold(((0., 0.), (0., 0.)), |(x_min, x_max), (&x, &y)| {
            let min_out = if x < x_min.0 { (x, y) } else { x_min };
            let max_out = if x > x_max.0 { (x, y) } else { x_max };
            (min_out, max_out)
        });

    let mut hull: Vec<f64> = Vec::new();

    qh_recursion(pointset, min_x, max_x, &mut hull);
    qh_recursion(pointset, max_x, min_x, &mut hull);

    // remove collinear points
    let first = (hull[0], hull[1]);
    let last = (hull[hull.len()-2], hull[hull.len()-1]);
    let mut hull = hull.iter()
        .tuples::<(_, _)>()
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| ((*a.0, *a.1), (*b.0, *b.1), (*c.0, *c.1)))
        .filter(|&i| cross2d(i.0, i.1, i.2) > 1e-6)
        .fold(Vec::new(), |mut acc, (_, b, _)| { acc.push(b.0); acc.push(b.1); acc });
    hull.push(last.0);
    hull.push(last.1);
    hull.push(first.0);
    hull.push(first.1);

    hull
}

fn qh_recursion(pointset: &[f64], a: (f64, f64), b: (f64, f64), out: &mut Vec<f64>) {
    // find left and farthest away point q
    let left_of: Vec<f64> = pointset.iter()
        .tuples::<(_, _)>()
        .map(|a| (*a.0, *a.1))
        .filter(|&i| cross2d(a, i, b) > 1e-6)
        .fold(Vec::new(), |mut acc, p| { acc.push(p.0); acc.push(p.1); acc });

    // if there is none: add b to out and return
    if left_of.len() == 0 {
        out.push(b.0);
        out.push(b.1);
    } else {
        // else recurse with the edge (a, q) and (q, b)
        let q = left_of.iter()
            .tuples::<(_, _)>()
            .map(|a| (*a.0, *a.1))
            .fold(b, |farthest: (f64, f64), i: (f64, f64)| if cross2d(a, farthest, b) > cross2d(a, i, b) {farthest} else {i});

        qh_recursion(&left_of, a, q, out);
        qh_recursion(&left_of, q, b, out);
    }
}

fn svg(pointset: &[f64], hull: &[f64]) {
    print!("<?xml version='1.0' encoding='UTF-8'?> \n\
                <!DOCTYPE svg PUBLIC '-//W3C//DTD SVG 1.1//EN' 'http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd'>\n\
                <svg xmlns='http://www.w3.org/2000/svg'\n\
                xmlns:xlink='http://www.w3.org/1999/xlink' xmlns:ev='http://www.w3.org/2001/xml-events'\n\
                version='1.1' baseProfile='full' width='800px' height='800px' viewBox='0 0 1 1'>\n");
    for i in pointset.iter().tuples::<(_, _)>() {
        println!("<circle cx='{}' cy='{}' r='0.01' stroke='black' stroke-width='0' />", i.0, i.1);
    }

    for (a, b) in hull.iter()
                      .tuples::<(_, _)>()
                      .tuple_windows::<(_, _)>()
    {
        println!("<line x1='{}' x2='{}' y1='{}' y2='{}' stroke='red' stroke-width='0.002' />", a.0, b.0, a.1, b.1);
    }
    println!("</svg>");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    extern crate rand;
    use self::rand::{StdRng, Rng, SeedableRng};

    fn get_test_vector() -> Vec<f64> {
        let seed: &[_] = &[42,];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        rng.gen_iter::<f64>()
           .take(2048 * 2)
           .collect()
    }

    #[test]
    fn test_hull() {
        let p = vec![
            0.0, 0.0,
            1.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
            0.5, 1.0,
            0.5, 0.5,
        ];

        let expected_area = 1.0;
        let hull_andrew = andrew(&p);
        let hull_qh = quickhull(&p);

        assert_eq!(hull_andrew.len(), 2*4);
        assert_eq!(hull_qh.len(), 2*4);
        assert_approx_eq!(area(&hull_andrew), expected_area);
        assert_approx_eq!(area(&hull_qh), expected_area);
    }

    #[bench]
    fn bench_andrew2048(b: &mut Bencher) {
        let v = get_test_vector();

        b.iter(|| andrew(&v));

        let hull = andrew(&v);
        assert_eq!(hull.len(), 36);
        assert_approx_eq!(area(&hull), 0.9942297515380842);
    }

    #[bench]
    fn bench_quickhull2048(b: &mut Bencher) {
        let v = get_test_vector();

        b.iter(|| quickhull(&v));

        let hull = quickhull(&v);
        assert_eq!(hull.len(), 36);
        assert_approx_eq!(area(&hull), 0.9942297515380842);
    }
}
