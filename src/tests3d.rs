use super::*;
use test::Bencher;

extern crate rand;
use self::rand::{StdRng, Rng, SeedableRng};

use itertools::Itertools;

fn get_test_vector(n: usize) -> Vec<Point3> {
    let seed: &[_] = &[42,];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    rng.gen_iter::<f64>()
       .map(|a| (a*100.))
       .take(n * 3)
       .tuples()
       .map(|(x, y, z)| Point3::new(x, y, z))
       .collect()
}

/// check that all points are behind every facet (or on)
fn is_convex(facets: &[Facet3], points: &[Point3]) -> bool {
    for f in facets {
        for p in points {
            if f.visible_from(p) {
                return false
            }
        }
    }

    true
}

#[test]
fn test_cube() {
    let p = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(0.0, 0.0, 1.0),
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(0.0, 1.0, 1.0),
        Point3::new(1.0, 1.0, 1.0),
    ];

    let expected_surface = 6.0;
    let hull_qh = quickhull3d(&p);

    // threejs(&p, &hull_qh, "cube.html");

    println!("{:?}", hull_qh);

    assert_eq!(hull_qh.len(), 12);
    assert_approx_eq!(surface(&hull_qh), expected_surface);
}

#[test]
fn test_cube2() {
    // cube and smaller cube inside
    let p = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(0.0, 0.0, 1.0),
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(0.0, 1.0, 1.0),
        Point3::new(1.0, 1.0, 1.0),

        Point3::new(0.1, 0.1, 0.1),
        Point3::new(0.9, 0.1, 0.1),
        Point3::new(0.1, 0.9, 0.1),
        Point3::new(0.9, 0.9, 0.1),
        Point3::new(0.1, 0.1, 0.9),
        Point3::new(0.9, 0.1, 0.9),
        Point3::new(0.1, 0.9, 0.9),
        Point3::new(0.9, 0.9, 0.9),

        // face centered
        Point3::new(0.9, 0.5, 0.5),
        Point3::new(0.1, 0.5, 0.5),
        Point3::new(0.5, 0.9, 0.5),
        Point3::new(0.5, 0.1, 0.5),
        Point3::new(0.5, 0.5, 0.9),
        Point3::new(0.5, 0.5, 0.1),

        // side centered
        Point3::new(0.9, 0.9, 0.5),
        Point3::new(0.1, 0.1, 0.5),
        Point3::new(0.9, 0.5, 0.9),
        Point3::new(0.1, 0.5, 0.1),
        Point3::new(0.5, 0.9, 0.9),
        Point3::new(0.5, 0.1, 0.1),
        Point3::new(0.5, 0.1, 0.5),
        Point3::new(0.5, 0.5, 0.9),
        Point3::new(0.5, 0.5, 0.1),
    ];

    let expected_surface = 6.0;
    let hull_qh = quickhull3d(&p);
    // threejs(&p, &hull_qh, "cube23d.html").expect("io error");
    println!("{:?}", hull_qh);


    assert_eq!(hull_qh.len(), 12);
    assert_approx_eq!(surface(&hull_qh), expected_surface);
}


#[test]
fn test_random() {
    let v = get_test_vector(200);
    let hull = quickhull3d(&v);
    assert!(is_convex(&hull, &v))
}


#[bench]
fn bench_quickhull3d_2048(b: &mut Bencher) {
    let v = get_test_vector(80);

    println!("start");

    b.iter(|| quickhull3d(&v));

    let hull = quickhull3d(&v);
    // threejs(&v, &hull, "quickhull3d.html").expect("io error");

    assert!(is_convex(&hull, &v));
    assert_eq!(hull.len(), 52);
    assert_approx_eq!(surface(&hull), 37311.19729514181);
}
