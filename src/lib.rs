#![feature(slice_patterns)]
#![feature(test)]
extern crate test;

#[macro_use]
extern crate assert_approx_eq;

use std::cmp::Ordering::{Less, Equal};

extern crate itertools;
use itertools::Itertools;

use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn cross2d(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

fn area(coord: &[f64]) -> f64 {
    coord.iter()
         .chain(coord.iter().take(2)) // append the first point, to close the loop
         .tuples::<(_, _)>()
         .tuple_windows::<(_, _)>()
         .fold(0f64, |sum, ((x1, y1), (x2, y2))| sum + (y1+y2) * (x1-x2)) / 2.
}

pub fn akl(pointset: &[f64]) -> Vec<f64> {
    let start = (pointset[0], pointset[1]);
    let octagon = pointset.iter()
        .tuples::<(_, _)>()
        .fold([start; 8], |[minx, minxmy, maxy, maxxpy, maxx, maxxmy, miny, minxpy], (&x, &y)| {
            let minx = if x < minx.0 { (x, y) } else { minx };
            let maxx = if x > maxx.0 { (x, y) } else { maxx };
            let miny = if y < miny.1 { (x, y) } else { miny };
            let maxy = if y > maxy.1 { (x, y) } else { maxy };
            let minxpy = if x+y < minxpy.0 + minxpy.1 { (x, y) } else { minxpy };
            let maxxpy = if x+y > maxxpy.0 + maxxpy.1 { (x, y) } else { maxxpy };
            let minxmy = if x-y < minxmy.0 - minxmy.1 { (x, y) } else { minxmy };
            let maxxmy = if x-y > maxxmy.0 - maxxmy.1 { (x, y) } else { maxxmy };
            [minx, minxmy, maxy, maxxpy, maxx, maxxmy, miny, minxpy]
        });

    pointset.iter()
        .tuples::<(_, _)>()
        .map(|a| (*a.0, *a.1))
        .filter(|&p| !point_in_octagon(octagon, p))
        .fold(Vec::new(), |mut acc, p| { acc.push(p.0); acc.push(p.1); acc })
}

fn side(p1: (f64, f64), p2: (f64, f64), p: (f64, f64)) -> f64 {
    return (p2.1 - p1.1)*(p.0 - p1.0) + (-p2.0 + p1.0)*(p.1 - p1.1);
}

fn point_in_octagon(octagon: [(f64, f64); 8], p: (f64, f64)) -> bool {
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

fn dist2(a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - b.0)*(a.0 - b.0) + (a.1 - b.1)*(a.1 - b.1)
}

// points stores a contiguous array of 2N floats in the format x1, y1, x2, y2, ...
pub fn andrew(pointset: &[f64]) -> Vec<f64> {
    // sort by x coordinates
    let sorted = pointset.iter()
        .tuples::<(_, _)>()
        .sorted_by(|a, b| {
            let tmp = a.0.partial_cmp(b.0).unwrap_or(Less);
            if tmp != Equal {
                tmp
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
        .tuples::<(_, _)>()
        .map(|a| (*a.0, *a.1))
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
            .tuples::<(_, _)>()
            .map(|a| (*a.0, *a.1))
            .fold(b, |farthest: (f64, f64), i: (f64, f64)| if cross2d(a, farthest, b) > cross2d(a, i, b) {farthest} else {i});

        qh_recursion(&left_of, a, q, out);
        qh_recursion(&left_of, q, b, out);
    }
}

fn svg(pointset: &[f64], hull: &[f64], filename: &str) -> Result<(), io::Error> {
    let path = Path::new(filename);

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path)?;

    write!(file, "<?xml version='1.0' encoding='UTF-8'?> \n\
                <!DOCTYPE svg PUBLIC '-//W3C//DTD SVG 1.1//EN' 'http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd'>\n\
                <svg xmlns='http://www.w3.org/2000/svg'\n\
                xmlns:xlink='http://www.w3.org/1999/xlink' xmlns:ev='http://www.w3.org/2001/xml-events'\n\
                version='1.1' baseProfile='full' width='800px' height='800px' viewBox='-0.05 -0.05 1.10 1.10'>\n")?;
    for i in pointset.iter().tuples::<(_, _)>() {
        write!(file, "<circle cx='{}' cy='{}' r='0.01' stroke='black' stroke-width='0' />\n", i.0, i.1)?;
    }

    for (a, b) in hull.iter()
                      .tuples::<(_, _)>()
                      .tuple_windows::<(_, _)>()
    {
        write!(file, "<line x1='{}' x2='{}' y1='{}' y2='{}' stroke='red' stroke-width='0.002' />\n", a.0, b.0, a.1, b.1)?;
    }
    write!(file, "</svg>\n")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    extern crate rand;
    use self::rand::{StdRng, Rng, SeedableRng};

    fn get_test_vector(n: usize) -> Vec<f64> {
        let seed: &[_] = &[42,];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        rng.gen_iter::<f64>()
           .take(n * 2)
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
    fn bench_andrew_2048(b: &mut Bencher) {
        let v = get_test_vector(2048);

        b.iter(|| andrew(&v));

        let hull = andrew(&v);
        svg(&v, &hull, "andrew.svg");

        assert_eq!(hull.len(), 48);
        assert_approx_eq!(area(&hull), 0.9915082733644154);
    }

    #[bench]
    fn bench_andrew_akl_2048(b: &mut Bencher) {
        let v = get_test_vector(2048);

        b.iter(|| andrew(&akl(&v)));

        let hull = andrew(&akl(&v));
        svg(&v, &hull, "andrew_akl.svg");

        assert_eq!(hull.len(), 48);
        assert_approx_eq!(area(&hull), 0.9915082733644154);
    }

    #[bench]
    fn bench_quickhull_2048(b: &mut Bencher) {
        let v = get_test_vector(2048);

        b.iter(|| quickhull(&v));

        let hull = quickhull(&v);
        svg(&v, &hull, "quickhull.svg");

        assert_eq!(hull.len(), 48);
        assert_approx_eq!(area(&hull), 0.9915082733644154);
    }

    #[bench]
    fn bench_quickhull_akl_2048(b: &mut Bencher) {
        let v = get_test_vector(2048);

        b.iter(|| quickhull(&akl(&v)));

        let hull = quickhull(&akl(&v));
        svg(&v, &hull, "quickhull_akl.svg");

        assert_eq!(hull.len(), 48);
        assert_approx_eq!(area(&hull), 0.9915082733644154);
    }
}
