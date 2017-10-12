#![feature(slice_patterns)]
#![feature(test)]
extern crate test;

#[macro_use]
extern crate assert_approx_eq;

extern crate itertools;

#[cfg(test)]
mod tests;

// 2d
mod primitives;
pub mod visualization;
pub mod andrew;
pub mod quickhull;
pub mod jarvis;
pub mod akl;

// 3d
mod d3;
pub mod quickhull3d;

// reexports:
use akl::akl;
use andrew::andrew;
use jarvis::jarvis;
use quickhull::quickhull;
use primitives::area;
use visualization::svg;
