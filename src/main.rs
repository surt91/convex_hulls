extern crate convex_hulls;
use convex_hulls::*;

extern crate rand;
use self::rand::{StdRng, Rng, SeedableRng};

extern crate itertools;
use itertools::Itertools;

fn get_test_vector_3d(n: usize) -> Vec<Point3> {
    let seed: &[_] = &[42,];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    rng.gen_iter::<f64>()
       .map(|a| (a*100.))
       .take(n * 3)
       .tuples()
       .map(|(x, y, z)| Point3::new(x, y, z))
       .collect()
}

fn get_test_vector(n: usize) -> Vec<f64> {
    let seed: &[_] = &[42,];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    rng.gen_iter::<f64>()
       .take(n * 2)
       .collect()
}

fn main() {
    let v = get_test_vector(10);
    convex_hulls::jarvis::jarvis(&v);
    convex_hulls::quickhull::quickhull(&v);
    convex_hulls::andrew::andrew(&v);
    let v = get_test_vector(27);
    convex_hulls::chan::chan(&v);

    let v = get_test_vector_3d(80);
    convex_hulls::quickhull3d::quickhull3d(&v);
}
