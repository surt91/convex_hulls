use d3::{Point3, Facet3, surface};

pub fn quickhull3d(pointset: &[Point3]) -> Vec<Facet3> {
    // get a facet with all points on the hull
    let start = pointset[0];
    // FIXME: we need to ensure that all 3 points are distinct
    let (max_x, max_y, max_z) = pointset.iter()
        .fold((start, start, start), |(max_x, max_y, max_z), &p| {
            let max_x = if p.x > max_x.x { p } else { max_x };
            let max_y = if p.y > max_y.y { p } else { max_y };
            let max_z = if p.z > max_z.z { p } else { max_z };
            (max_x, max_y, max_z)
        });

    let mut hull: Vec<Facet3> = Vec::new();

    println!("first");
    quickhull3d_recursion(pointset, Facet3 { vertices: [max_x, max_y, max_z] }, &mut hull);
    println!("second");
    quickhull3d_recursion(pointset, Facet3 { vertices: [max_x, max_z, max_y] }, &mut hull);

    hull
}

fn quickhull3d_recursion(candidates: &[Point3], facet: Facet3, out: &mut Vec<Facet3>) {
    let normal = facet.normal();
    let p: Point3 = facet.vertices[0];
    let in_front_of: Vec<Point3> = candidates.iter()
        .cloned()
        .filter(|&i| normal.dot(i-p) > 0f64)
        .collect();

    println!("look at: {:?}", facet);
    println!("normal: {:?}", normal);
    println!("candidates: {:?}", candidates);
    println!("in front: {:?}", in_front_of);

    // if there is none: add it to out and return
    if in_front_of.len() == 0 {
        println!("add: {:?}", facet);
        out.push(facet);
    } else {
        // else recurse with the three edges constructable from 2 original points and the farthest point
        // pay attention to orientation: normal must point outside
        let q = in_front_of.iter()
            .cloned()
            .fold(p, |farthest: Point3, i: Point3| if normal.dot(farthest-p) > normal.dot(i-p) {farthest} else {i});

        println!("farthest: {:?}", q);
        println!("diff: {:?}", q-p);

        let p1: Point3 = facet.vertices[0];
        let p2: Point3 = facet.vertices[1];
        let p3: Point3 = facet.vertices[2];

        let f1 = Facet3 { vertices: [p1, p2, q] };
        let f2 = Facet3 { vertices: [p2, p3, q] };
        let f3 = Facet3 { vertices: [p3, p1, q] };

        quickhull3d_recursion(&in_front_of, f1, out);
        quickhull3d_recursion(&in_front_of, f2, out);
        quickhull3d_recursion(&in_front_of, f3, out);
    }
}

#[test]
fn test_hull3d() {
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

    println!("{:?}", hull_qh);


    assert_eq!(hull_qh.len(), 12);
    assert_approx_eq!(surface(&hull_qh), expected_surface);
}
