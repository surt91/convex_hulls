use convex_hulls::{util::{get_test_vector_3d, get_test_vector_2d}};

fn main() {
    let v = get_test_vector_2d(10);
    convex_hulls::jarvis::jarvis(&v);
    convex_hulls::quickhull::quickhull(&v);
    convex_hulls::andrew::andrew(&v);
    let v = get_test_vector_2d(27);
    convex_hulls::chan::chan(&v);

    let v = get_test_vector_3d(80);
    convex_hulls::quickhull3d::quickhull3d(&v);
}
