use convex_hulls::{andrew, akl, quickhull, jarvis, chan, quickhull3d};
use convex_hulls::util::{get_test_vector_2d, get_test_vector_3d};
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_andrew_2048(c: &mut Criterion) {
    let v = get_test_vector_2d(2048);

    c.bench_function("andrew", |b| b.iter(|| andrew(&v)));
}

fn bench_andrew_akl_2048(c: &mut Criterion) {
    let v = get_test_vector_2d(2048);

    c.bench_function("andrew+akl", |b| b.iter(|| andrew(&akl(&v))));
}

fn bench_quickhull_2048(c: &mut Criterion) {
    let v = get_test_vector_2d(2048);

    c.bench_function("quickhull", |b| b.iter(|| quickhull(&v)));
}

fn bench_quickhull_akl_2048(c: &mut Criterion) {
    let v = get_test_vector_2d(2048);

    c.bench_function("quickhull+akl", |b| b.iter(|| quickhull(&akl(&v))));
}

fn bench_jarvis_2048(c: &mut Criterion) {
    let v = get_test_vector_2d(2048);

    c.bench_function("jarvis", |b| b.iter(|| jarvis(&v)));
}

fn bench_jarvis_akl_2048(c: &mut Criterion) {
    let v = get_test_vector_2d(2048);

    c.bench_function("jarvis+akl", |b| b.iter(|| jarvis(&akl(&v))));
}

fn bench_chan_2048(c: &mut Criterion) {
    let v = get_test_vector_2d(2048);

    c.bench_function("chan", |b| b.iter(|| chan(&v)));
}

fn bench_chan_akl_2048(c: &mut Criterion) {
    let v = get_test_vector_2d(2048);

    c.bench_function("chan+akl", |b| b.iter(|| chan(&akl(&v))));
}

fn bench_quickhull3d_2048(c: &mut Criterion) {
    let v = get_test_vector_3d(80);

    c.bench_function("quickhull 3d", |b| b.iter(|| quickhull3d(&v)));
}

criterion_group!{
    name = benches;
    config = Criterion::default();
    targets =
        bench_andrew_2048,
        bench_andrew_akl_2048,
        bench_quickhull_2048,
        bench_quickhull_akl_2048,
        bench_jarvis_2048,
        bench_jarvis_akl_2048,
        bench_chan_2048,
        bench_chan_akl_2048,
        bench_quickhull3d_2048,
}

criterion_main!(benches);