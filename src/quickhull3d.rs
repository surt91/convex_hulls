use itertools::Itertools;

use d3::{Point3, Facet3, Edge3, surface, threejs};

pub fn quickhull3d(pointset: &[Point3]) -> Vec<Facet3> {
    // get a facet with all points on the hull
    let start = pointset[0];
    // FIXME: we need to ensure that all 3 points are distinct
    let extrema = pointset.iter()
        .fold([start; 6], |[max_x, max_y, max_z, min_x, min_y, min_z], &p| {
            let max_x = if p.x > max_x.x { p } else { max_x };
            let max_y = if p.y > max_y.y { p } else { max_y };
            let max_z = if p.z > max_z.z { p } else { max_z };
            let min_x = if p.x < min_x.x { p } else { min_x };
            let min_y = if p.y < min_y.y { p } else { min_y };
            let min_z = if p.z < min_z.z { p } else { min_z };
            [max_x, max_y, max_z, min_x, min_y, min_z]
        });

    let mut hull: Vec<Facet3> = Vec::new();

    let mut unique: Vec<Point3> = Vec::new();
    'outer: for i in 0..extrema.len() {
        for j in &unique {
            if *j == extrema[i] {
                continue 'outer;
            }
        }
        unique.push(extrema[i]);
    }
    if unique.len() < 3 {
        panic!();
    }

    let p1 = unique[0];
    let p2 = unique[1];
    let p3 = unique[2];

    // println!("all points {:?}", pointset);

    let mut ctr = 0;
    let f = Facet3 { vertices: [p1, p2, p3] };
    let q = farthest(&f, pointset);

    let f1 = Facet3 { vertices: [p1, p3, p2] };
    let f2 = Facet3 { vertices: [p1, p2, q] };
    let f3 = Facet3 { vertices: [p2, p3, q] };
    let f4 = Facet3 { vertices: [p3, p1, q] };
    let facets = [f1.clone(), f2.clone(), f3.clone(), f4.clone()];

    // initial tetrahedron
    hull.push(f1.clone());
    hull.push(f2.clone());
    hull.push(f3.clone());
    hull.push(f4.clone());

    let candidates = divide_points_to_facets(&pointset, &facets);

    println!("start 1 {:?}", candidates[1].len());
    println!("start 2 {:?}", candidates[2].len());
    println!("start 3 {:?}", candidates[3].len());
    println!("start 4 {:?}", candidates[4].len());

    // FIXME do not give the whole pointset but disjunct subsets
    println!("1");
    quickhull3d_recursion(&candidates[1], &f1, &mut hull, pointset, &mut ctr);
    println!("2");
    quickhull3d_recursion(&candidates[2], &f2, &mut hull, pointset, &mut ctr);
    println!("3");
    quickhull3d_recursion(&candidates[3], &f3, &mut hull, pointset, &mut ctr);
    println!("4");
    quickhull3d_recursion(&candidates[4], &f4, &mut hull, pointset, &mut ctr);

    hull
}

fn divide_points_to_facets(pointset: &[Point3], facets: &[Facet3]) -> Vec<Vec<Point3>> {
    let mut candidates: Vec<Vec<Point3>> = vec![Vec::new(); facets.len() + 1];

    for p in pointset {
        let mut min_facet = 0;
        let mut min_distance = 1e10;
        for (n, f) in facets.iter().enumerate() {
            // TODO can be precomputed
            if !f.visible_from(p) {
                continue
            }

            let m = f.mid();
            let normal = f.normal();
            let d = normal.dot(*p-m);
            if d < min_distance {
                min_distance = d;
                min_facet = n + 1;
            }
        }
        candidates[min_facet].push(p.clone());
    }

    candidates
}

fn farthest(facet: &Facet3, candidates: &[Point3]) -> Point3 {
    let p = facet.mid();
    let normal = facet.normal();
    candidates.iter()
        .cloned()
        .fold(
            p,
            |farthest: Point3, i: Point3|
                if normal.dot(farthest-p) > normal.dot(i-p) {
                    farthest
                } else {
                    i
                }
        )
}

fn get_candidates_multiple(facets: &[Facet3], candidates: &[Point3]) -> Vec<Point3> {
    candidates.iter()
        .cloned()
        .filter(|i| facets.iter().any(|f| f.visible_from(i)))
        .collect()
}

fn get_candidates(facet: &Facet3, candidates: &[Point3]) -> Vec<Point3> {
    candidates.iter()
        .cloned()
        .filter(|i| facet.visible_from(i))
        .collect()
}

fn quickhull3d_recursion(candidates: &[Point3], facet: &Facet3, out: &mut Vec<Facet3>, all_points: &[Point3], ctr: &mut usize) {
    let normal = facet.normal();
    let p = facet.mid();
    let in_front_of = get_candidates(facet, candidates);

    println!("\n#: {:?}", ctr);
    println!("look at: {:?}", facet);
    println!("mid: {:?}", p);
    println!("normal: {:?}", normal);
    println!("candidates: {} {:?}", candidates.len(), candidates);
    println!("in front: {} {:?}", in_front_of.len(), in_front_of);

    // if there are still candidates continue, else we are finished
    if in_front_of.len() != 0 {
        // pay attention to orientation: normal must point outside
        let q = farthest(facet, &in_front_of);

        println!("farthest: {:?}", q);
        println!("diff: {:?}", q-p);
        println!("out: {} {:?}", out.len(), out);

        // TODO: q is eye point, search for the horizon
        // delete all facets inside the horizon
        // make facets from the horizon edges to the eye point

        // FIXME: this implementation is slow and should be replaced by a DFS or similar
        // FIXME: to speed it up slightly, replace the Vec out by a set or something
        // Just test for every facet if it is visible from the eye point,
        // i.e., if it is, remove it and remember all edges
        // every edge which is remembered only once, is part of the horizon
        let mut visible_facets: Vec<Facet3> = Vec::new();
        for f in out.iter() {
            if f.visible_from(&q) {
                visible_facets.push(f.clone())
            }
        }

        println!("visible: {} {:?}", visible_facets.len(), visible_facets);

        // extract all edges (in correct orientation) from the visible facets
        let mut all_edges: Vec<Edge3> = Vec::new();
        for f in visible_facets.iter() {
            let v1 = f.vertices[0];
            let v2 = f.vertices[1];
            let v3 = f.vertices[2];
            all_edges.push(Edge3 { vertices: [v1, v2] });
            all_edges.push(Edge3 { vertices: [v2, v3] });
            all_edges.push(Edge3 { vertices: [v3, v1] });
            out.remove_item(f);
        }

        println!("edges: {:?}", all_edges);

        let mut horizon: Vec<Edge3> = Vec::new();
        for i in all_edges.iter() {
            if all_edges.iter().filter(|&x| x == i).count() == 1 {
                horizon.push(i.clone());
            }
        }

        println!("horizon: {} {:?}", horizon.len(), horizon);

        *ctr += 1;
        threejs(&all_points, &out, &q, &in_front_of, &visible_facets, &horizon, &format!("quickhull3d_{}.html", ctr)).expect("io error");
        // threejs(&in_front_of, &out, &horizon, &format!("quickhull3d_{}.html", ctr)).expect("io error");

        // facets generated in this iteration of the recursion
        let mut new_facets = Vec::new();
        for e in horizon.iter() {
            let f = Facet3 { vertices: [e.vertices[0], e.vertices[1], q] };
            out.push(f.clone());
            new_facets.push(f);
        }

        // calculate for every candidate point the nearest facet
        // this way every point will only occur in one subtree of the recursion
        // FIXME we are testing far too many points. it we should discard interior points
        let possible = get_candidates_multiple(&new_facets, &all_points);
        let candidates = divide_points_to_facets(&possible, &new_facets);

        for (n, f) in new_facets.iter().enumerate() {
            quickhull3d_recursion(&candidates[n+1], &f, out, all_points, ctr);
        }
    }
}
