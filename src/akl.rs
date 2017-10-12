use itertools::Itertools;

use primitives::point_in_octagon;

pub fn akl(pointset: &[f64]) -> Vec<f64> {
    let start = (pointset[0], pointset[1]);
    let octagon = pointset.iter()
        .tuples::<(_, _)>()
        .fold([start; 8], |[minx, minxmy, maxy, maxxpy, maxx, maxxmy, miny, minxpy], (&x, &y)| {
            let minx = if x < minx.0 { (x, y) } else { minx };
            let maxx = if x > maxx.0 { (x, y) } else { maxx };
            let miny = if y < miny.1 { (x, y) } else { miny };
            let maxy = if y > maxy.1 { (x, y) } else { maxy };
            let minxpy = if x+y < minxpy.0 + minxpy.1 { (x, y) } else { minxpy };
            let maxxpy = if x+y > maxxpy.0 + maxxpy.1 { (x, y) } else { maxxpy };
            let minxmy = if x-y < minxmy.0 - minxmy.1 { (x, y) } else { minxmy };
            let maxxmy = if x-y > maxxmy.0 - maxxmy.1 { (x, y) } else { maxxmy };
            [minx, minxmy, maxy, maxxpy, maxx, maxxmy, miny, minxpy]
        });

    pointset.iter()
        .cloned()
        .tuples::<(_, _)>()
        .filter(|&p| !point_in_octagon(octagon, p))
        .fold(Vec::new(), |mut acc, p| { acc.push(p.0); acc.push(p.1); acc })
}
