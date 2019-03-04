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
    let hull_jarvis = jarvis(&p);
    let hull_chan = chan(&p);

    assert_eq!(hull_andrew.len(), 2*4);
    assert_eq!(hull_qh.len(), 2*4);
    assert_eq!(hull_jarvis.len(), 2*4);
    assert_eq!(hull_chan.len(), 2*4);
    assert_approx_eq!(area(&hull_andrew), expected_area);
    assert_approx_eq!(area(&hull_qh), expected_area);
    assert_approx_eq!(area(&hull_jarvis), expected_area);
    assert_approx_eq!(area(&hull_chan), expected_area);
}

#[bench]
fn bench_andrew_2048(b: &mut Bencher) {
    let v = get_test_vector(2048);

    b.iter(|| andrew(&v));

    let hull = andrew(&v);
    #[cfg(feature = "visual")] svg(&v, &hull, "andrew.svg").expect("io error");

    assert_eq!(hull.len(), 48);
    assert_approx_eq!(area(&hull), 0.9915082733644154);
}

#[bench]
fn bench_andrew_akl_2048(b: &mut Bencher) {
    let v = get_test_vector(2048);

    b.iter(|| andrew(&akl(&v)));

    let hull = andrew(&akl(&v));
    #[cfg(feature = "visual")] svg(&v, &hull, "andrew_akl.svg").expect("io error");

    assert_eq!(hull.len(), 48);
    assert_approx_eq!(area(&hull), 0.9915082733644154);
}

#[bench]
fn bench_quickhull_2048(b: &mut Bencher) {
    let v = get_test_vector(2048);

    b.iter(|| quickhull(&v));

    let hull = quickhull(&v);
    #[cfg(feature = "visual")] svg(&v, &hull, "quickhull.svg").expect("io error");

    assert_eq!(hull.len(), 48);
    assert_approx_eq!(area(&hull), 0.9915082733644154);
}

#[bench]
fn bench_quickhull_akl_2048(b: &mut Bencher) {
    let v = get_test_vector(2048);

    b.iter(|| quickhull(&akl(&v)));

    let hull = quickhull(&akl(&v));
    #[cfg(feature = "visual")] svg(&v, &hull, "quickhull_akl.svg").expect("io error");

    assert_eq!(hull.len(), 48);
    assert_approx_eq!(area(&hull), 0.9915082733644154);
}

#[bench]
fn bench_jarvis_2048(b: &mut Bencher) {
    let v = get_test_vector(2048);

    b.iter(|| jarvis(&v));

    let hull = jarvis(&v);
    #[cfg(feature = "visual")] svg(&v, &hull, "jarvis_akl.svg").expect("io error");

    assert_eq!(hull.len(), 48);
    assert_approx_eq!(area(&hull), 0.9915082733644154);
}

#[bench]
fn bench_jarvis_akl_2048(b: &mut Bencher) {
    let v = get_test_vector(2048);

    b.iter(|| jarvis(&akl(&v)));

    let hull = jarvis(&akl(&v));
    #[cfg(feature = "visual")] svg(&v, &hull, "jarvis_akl.svg").expect("io error");

    assert_eq!(hull.len(), 48);
    assert_approx_eq!(area(&hull), 0.9915082733644154);
}

#[bench]
fn bench_chan_2048(b: &mut Bencher) {
    let v = get_test_vector(2048);

    b.iter(|| chan(&v));

    let hull = chan(&v);
    #[cfg(feature = "visual")] svg(&v, &hull, "chan.svg").expect("io error");

    assert_eq!(hull.len(), 48);
    assert_approx_eq!(area(&hull), 0.9915082733644154);
}

#[bench]
fn bench_chan_akl_2048(b: &mut Bencher) {
    let v = get_test_vector(2048);

    b.iter(|| chan(&akl(&v)));

    let hull = chan(&akl(&v));
    #[cfg(feature = "visual")] svg(&v, &hull, "chan_akl.svg").expect("io error");

    assert_eq!(hull.len(), 48);
    assert_approx_eq!(area(&hull), 0.9915082733644154);
}
