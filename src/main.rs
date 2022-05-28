use convex_hulls::util::{get_test_vector_gaussian, get_test_vector_gaussian_3d};

fn main() {
    let v = get_test_vector_gaussian(300);
    convex_hulls::chan(&v);
    convex_hulls::jarvis(&v);
    convex_hulls::andrew(&v);
    convex_hulls::quickhull(&v);

    let v = get_test_vector_gaussian_3d(300);
    convex_hulls::quickhull3d(&v);
}
