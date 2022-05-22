use rand::{StdRng, Rng, SeedableRng};
use itertools::Itertools;
use crate::Point3;


#[cfg(feature = "visual")]
use crate::svg;

pub fn get_test_vector_2d(n: usize) -> Vec<f64> {
    let seed: &[_] = &[42,];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    rng.gen_iter::<f64>()
    .take(n * 2)
    .collect()
}

pub fn get_test_vector_3d(n: usize) -> Vec<Point3> {
    let seed: &[_] = &[42,];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    rng.gen_iter::<f64>()
    .map(|a| (a*100.))
    .take(n * 3)
    .tuples()
    .map(|(x, y, z)| Point3::new(x, y, z))
    .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;

    use crate::Facet3;
    use crate::Point3;

    use crate::d3::is_convex;
    use crate::{area, surface};

    #[cfg(feature = "visual")]
    use crate::svg;

    const TEST_AREA_2D: f64 = 0.9915082733644154;
    const TEST_AREA_2D_POINTS: usize = 48;
    const TEST_AREA_3D: f64 = 37311.19729514181;
    const TEST_AREA_3D_POINTS: usize = 52;

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

    pub(crate) fn check_3d_80(algo: fn(&[Point3]) -> Vec<Facet3>, name: &str) {
        let v = get_test_vector_3d(80);

        let hull = algo(&v);
        //#[cfg(feature = "visual")] threejs(&v, &hull, "quickhull3d.html").expect("io error");

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