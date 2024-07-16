//  LIB.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 00:06:19
//  Last edited:
//    16 Jul 2024, 20:41:44
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the different implementations of K-Shortest Path (KSP)
//!   algorithms.
//

// Declare modules
pub mod graph;
pub mod path;
#[cfg(feature = "peek")]
pub mod peek;
#[cfg(feature = "vanilla")]
pub mod vanilla;

// Imports
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::str::FromStr;

// Use some of it in this namespace
pub use graph::*;
pub use path::*;


/***** ERRORS *****/
/// Defines the error thrown when an unknown [`Algorithm`] was parsed.
#[derive(Debug)]
pub struct UnknownAlgorithmError {
    /// The raw string that wasn't a recongized algorithm.
    pub unknown: String,
}
impl Display for UnknownAlgorithmError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult { write!(f, "Unknown KSP algorithm '{}'", self.unknown) }
}
impl Error for UnknownAlgorithmError {}





/***** LIBRARY *****/
/// Overview of all algorithms in the libary.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Algorithm {
    /// The default, simplest version of a KSP-algorithm.
    #[cfg(feature = "vanilla")]
    Vanilla,
}
impl FromStr for Algorithm {
    type Err = UnknownAlgorithmError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vanilla" => Ok(Self::Vanilla),
            other => Err(UnknownAlgorithmError { unknown: other.into() }),
        }
    }
}



/// Defines an abstraction over various algorithms.
pub trait Routing {
    /// Finds The K shortest paths from one node to another.
    ///
    /// # Arguments
    /// - `graph`: The [`Graph`] to find in.
    /// - `src`: The source node to find a path from.
    /// - `dst`: The destination node to find a path to.
    /// - `k`: The number of paths to find.
    ///
    /// # Returns
    /// A list of the shortest paths found. Is at most `k` elements long.
    ///
    /// # Panics
    /// This function is allowed to panic if the given `src` or `dst` are not in the given `graph`.
    fn k_shortest_paths<'g>(&mut self, graph: &'g graph::Graph, src: &str, dst: &str, k: usize) -> Vec<Path<'g>>;
}
