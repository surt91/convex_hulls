#![feature(test)]
extern crate test;

#[cfg(test)]
#[macro_use]
extern crate assert_approx_eq;

extern crate itertools;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests3d;

#[cfg(feature = "visual")]
pub mod visualization;

// 2d
mod primitives;
pub mod andrew;
pub mod quickhull;
pub mod jarvis;
pub mod chan;
pub mod akl;

// 3d
mod d3;
pub mod quickhull3d;

// reexports:
pub use akl::akl;
pub use andrew::andrew;
pub use jarvis::jarvis;
pub use chan::chan;
pub use quickhull::quickhull;
pub use quickhull3d::quickhull3d;
pub use primitives::area;
pub use d3::{surface, Point3, Facet3};
#[cfg(feature = "visual")] pub use d3::threejs;
#[cfg(feature = "visual")] pub use visualization::svg;
