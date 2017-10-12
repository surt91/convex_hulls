use std::ops::{Sub, DivAssign};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point3 {
    pub fn cross(&self, other: Point3) -> Point3 {
        let x = self.y*other.z - other.y*self.z;
        let y = self.z*other.x - other.z*self.x;
        let z = self.x*other.y - other.x*self.y;
        Point3 {x, y, z}
    }

    pub fn dot(self, other: Point3) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 {x, y, z}
    }
}

impl Sub for Point3 {
    type Output = Point3;

    fn sub(self, other: Point3) -> Point3 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Point3 {x, y, z}
    }
}

impl DivAssign<f64> for Point3 {
    fn div_assign(&mut self, divisor: f64) {
        self.x /= divisor;
        self.y /= divisor;
        self.z /= divisor;
    }
}

#[derive(Debug, PartialEq)]
pub struct Facet3 {
    pub vertices: [Point3; 3],
}

impl Facet3 {
    pub fn normal(&self) -> Point3 {
        // let mut n = (self.vertices[1] - self.vertices[0]).cross(self.vertices[2] - self.vertices[0]);
        // n /= n.length();
        // n
        // i dont think i need it normalized
        (self.vertices[1] - self.vertices[0]).cross(self.vertices[2] - self.vertices[0])
    }

    pub fn surface(&self) -> f64 {
        (self.vertices[1] - self.vertices[0]).cross(self.vertices[2] - self.vertices[0]).length()/2.
    }
}

pub fn surface(facets: &[Facet3]) -> f64 {
    facets.iter().map(|f| f.surface()).sum()
}
