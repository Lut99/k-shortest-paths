//  MOD.rs
//    by Lut99
//
//  Created:
//    24 Jul 2024, 01:44:45
//  Last edited:
//    24 Jul 2024, 01:54:04
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the various K-shortest Path algorithms.
//

pub mod wikipedia;
pub mod yen;

// Imports
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::str::FromStr;

use ksp_graph::Graph;

use crate::path::Path;


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
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Algorithm {
    /// The default, simplest version of a KSP-algorithm as presented by Wikipedia.
    Wikipedia,
    /// The default, simplest version of a KSP-algorithm as presented by the `PeeK`-paper [1].
    Yen,
}
impl Algorithm {
    /// Returns all implemented algorithms.
    ///
    /// # Returns
    /// A static list of the implemented algorithms.
    #[inline]
    pub const fn all() -> &'static [Self] { &[Self::Wikipedia, Self::Yen] }
}
impl FromStr for Algorithm {
    type Err = UnknownAlgorithmError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "wikipedia" => Ok(Self::Wikipedia),
            "yen" => Ok(Self::Yen),
            other => Err(UnknownAlgorithmError { unknown: other.into() }),
        }
    }
}



/// Defines an abstraction over various algorithms.
pub trait KShortestPath {
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
    fn k_shortest_paths<'g>(graph: &'g Graph, src: &str, dst: &str, k: usize) -> Vec<Path<'g>>;
}
