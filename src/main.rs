use convex_hulls::util::get_test_vector_gaussian;

fn main() {
    let v = get_test_vector_gaussian(300);
    convex_hulls::chan::chan(&v);
    convex_hulls::jarvis::jarvis(&v);
    convex_hulls::andrew::andrew(&v);
    convex_hulls::quickhull::quickhull(&v);
}
