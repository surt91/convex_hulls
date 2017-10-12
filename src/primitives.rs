use itertools::Itertools;

pub fn side(p1: (f64, f64), p2: (f64, f64), p: (f64, f64)) -> f64 {
    return (p2.1 - p1.1)*(p.0 - p1.0) + (-p2.0 + p1.0)*(p.1 - p1.1);
}

pub fn point_in_octagon(octagon: [(f64, f64); 8], p: (f64, f64)) -> bool {
    // compare opposite sites first
       side(octagon[0], octagon[1], p) > 0f64
    && side(octagon[2], octagon[3], p) > 0f64
    && side(octagon[4], octagon[5], p) > 0f64
    && side(octagon[6], octagon[7], p) > 0f64
    && side(octagon[1], octagon[2], p) > 0f64
    && side(octagon[3], octagon[4], p) > 0f64
    && side(octagon[5], octagon[6], p) > 0f64
    && side(octagon[7], octagon[0], p) > 0f64
}

pub fn dist2(a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - b.0)*(a.0 - b.0) + (a.1 - b.1)*(a.1 - b.1)
}

pub fn cross2d(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

pub fn area(coord: &[f64]) -> f64 {
    coord.iter()
         .chain(coord.iter().take(2)) // append the first point, to close the loop
         .tuples::<(_, _)>()
         .tuple_windows::<(_, _)>()
         .fold(0f64, |sum, ((x1, y1), (x2, y2))| sum + (y1+y2) * (x1-x2)) / 2.
}
