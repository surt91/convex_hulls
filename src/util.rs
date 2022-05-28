use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use rand_distr::{Normal, Uniform};
use itertools::Itertools;
use crate::{Point3, Facet3};

pub fn get_test_vector_2d(n: usize) -> Vec<f64> {
    let seed = 42;
    let rng: Pcg64 = SeedableRng::seed_from_u64(seed);
    let uniform = Uniform::new(0.0, 1.0);
    rng.sample_iter(uniform)
        .take(n * 2)
        .collect()
}

pub fn get_test_vector_gaussian(n: usize) -> Vec<f64> {
    let seed = 42;
    let mut rng1: Pcg64 = SeedableRng::seed_from_u64(seed);
    let rng2: Pcg64 = SeedableRng::seed_from_u64(rng1.gen());
    let normal1 = Normal::new(0.0, 2.0).unwrap();
    let normal2 = Normal::new(0.0, 3.0).unwrap();
    let positions: Vec<f64> = rng2.sample_iter(normal2)
        .take(n)
        .interleave(
            (&mut rng1)
                .sample_iter(normal1)
                .take(n)
        )
        .collect();
    let scale = positions.iter()
        .map(|x| x.abs())
        .reduce(f64::max)
        .unwrap();
    positions.iter()
        .map(|x| x / scale / 2. + 0.5)
        .collect()
}

pub fn get_test_vector_3d(n: usize) -> Vec<Point3> {
    let seed = 42;
    let rng: Pcg64 = SeedableRng::seed_from_u64(seed);
    let uniform = Uniform::new(0.0, 1.0);
    rng.sample_iter(uniform)
        .map(|a| (a*100.))
        .take(n * 3)
        .tuples()
        .map(|(x, y, z)| Point3::new(x, y, z))
        .collect()
}

pub fn get_test_vector_gaussian_3d(n: usize) -> Vec<Point3> {
    let seed = 42;
    let rng: Pcg64 = SeedableRng::seed_from_u64(seed);
    let normal = Normal::new(0.0, 0.2).unwrap();
    rng.sample_iter(normal)
        .map(|a| (a*100.))
        .take(n * 3)
        .tuples()
        .map(|(x, y, z)| Point3::new(x, y, z))
        .collect()
}

/// check that all points are behind every facet (or on)
pub fn is_convex(facets: &[Facet3], points: &[Point3]) -> bool {
    for f in facets {
        for p in points {
            if f.visible_from(p) {
                return false
            }
        }
    }

    true
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;

    use crate::Facet3;
    use crate::Point3;

    use crate::{area, surface};

    #[cfg(feature = "visual")]
    use crate::svg;

    const TEST_AREA_2D: f64 = 0.9887111601582999;
    const TEST_AREA_2D_POINTS: usize = 38;
    const TEST_AREA_3D: f64 = 40322.038417123375;
    const TEST_AREA_3D_POINTS: usize = 50;

    pub(crate) fn check_2048(algo: fn(&[f64]) -> Vec<f64>, name: &str) {
        let v = get_test_vector_2d(2048);

        let hull = algo(&v);
        #[cfg(feature = "visual")] svg(&v, &hull, name).expect("io error");

        assert_eq!(hull.len(), TEST_AREA_2D_POINTS);
        assert_approx_eq!(area(&hull), TEST_AREA_2D);
    }

    pub(crate) fn check_square(algo: fn(&[f64]) -> Vec<f64>) {
        let v = get_square();

        let hull = algo(&v);

        assert_eq!(hull.len(), 8);
        assert_approx_eq!(area(&hull), 1.0);
    }


    pub(crate) fn get_square() -> Vec<f64> {
        vec![
            0.0, 0.0,
            1.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
            0.5, 1.0,
            0.5, 0.5,
        ]
    }

    pub(crate) fn check_3d_80(algo: fn(&[Point3]) -> Vec<Facet3>) {
        let v = get_test_vector_3d(80);

        let hull = algo(&v);

        assert!(is_convex(&hull, &v));
        assert_eq!(hull.len(), TEST_AREA_3D_POINTS);
        assert_approx_eq!(surface(&hull), TEST_AREA_3D);

    }

    pub(crate) fn check_simple_cube(algo: fn(&[Point3]) -> Vec<Facet3>) {
        let v = get_simple_cube();

        let hull = algo(&v);

        assert!(is_convex(&hull, &v));
        assert_eq!(hull.len(), 12);
        assert_approx_eq!(surface(&hull), 6.0);
    }

    pub(crate) fn check_cube(algo: fn(&[Point3]) -> Vec<Facet3>) {
        let v = get_cube();

        let hull = algo(&v);

        assert!(is_convex(&hull, &v));
        assert_eq!(hull.len(), 12);
        assert_approx_eq!(surface(&hull), 6.0);
    }

    pub(crate) fn get_simple_cube() -> Vec<Point3> {
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(1.0, 0.0, 1.0),
            Point3::new(0.0, 1.0, 1.0),
            Point3::new(1.0, 1.0, 1.0),
        ]
    }

    pub(crate) fn get_cube() -> Vec<Point3> {
        // cube and smaller cube inside
        vec![
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
        ]
    }
}