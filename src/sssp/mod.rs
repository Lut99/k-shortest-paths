//  MOD.rs
//    by Lut99
//
//  Created:
//    24 Jul 2024, 00:41:28
//  Last edited:
//    24 Jul 2024, 20:54:18
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines a few Single-Source ShortestPath (SSSP) algorithms.
//

// Declarations
pub mod dijkstra;
pub mod profiled;

// Imports
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::str::FromStr;

use ksp_graph::Graph;

use crate::path::Path;


/***** ERRORS *****/
/// Defines the error thrown when an unknown [`Sssp`] was parsed.
#[derive(Debug)]
pub struct UnknownSsspError {
    /// The raw string that wasn't a recongized SSSP algorithm.
    pub unknown: String,
}
impl Display for UnknownSsspError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult { write!(f, "Unknown SSSP algorithm '{}'", self.unknown) }
}
impl Error for UnknownSsspError {}





/***** LIBRARY *****/
/// Overview of all SSSP algorithms in the libary.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Sssp {
    /// Arguably the most famous one from Dijkstra ([2]).
    Dijkstra,
}
impl Sssp {
    /// Returns all implemented SSSP algorithms.
    ///
    /// # Returns
    /// A static list of the implemented SSSP algorithms.
    #[inline]
    pub const fn all() -> &'static [Self] { &[Self::Dijkstra] }
}
impl FromStr for Sssp {
    type Err = UnknownSsspError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dijkstra" => Ok(Self::Dijkstra),
            other => Err(UnknownSsspError { unknown: other.into() }),
        }
    }
}





/***** LIBRARY *****/
/// Defines an abstraction over various algorithms.
pub trait SingleShortestPath {
    /// Finds the shortest paths from one node to another.
    ///
    /// # Arguments
    /// - `graph`: The [`Graph`] to find in.
    /// - `src`: The source node to find a path from.
    /// - `dst`: The destination node to find a path to.
    ///
    /// # Returns
    /// The shortest paths found.
    ///
    /// # Panics
    /// This function is allowed to panic if the given `src` or `dst` are not in the given `graph` or they are not connected.
    fn shortest<'g>(&mut self, graph: &'g Graph, src: &str, dst: &str) -> Path<'g>;
}

// Pointer-like impls
impl<'a, T: SingleShortestPath> SingleShortestPath for &'a mut T {
    #[inline]
    fn shortest<'g>(&mut self, graph: &'g Graph, src: &str, dst: &str) -> Path<'g> { <T as SingleShortestPath>::shortest(self, graph, src, dst) }
}
