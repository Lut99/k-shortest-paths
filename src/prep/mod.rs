//  MOD.rs
//    by Lut99
//
//  Created:
//    24 Jul 2024, 01:48:03
//  Last edited:
//    24 Jul 2024, 02:05:42
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines preprocessing steps for K-Shortest Path algorithms.
//

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::str::FromStr;

use ksp_graph::Graph;

// Declare the modules
pub mod peek;


/***** ERRORS *****/
/// Defines the error thrown when an unknown [`Step`] was parsed.
#[derive(Debug)]
pub struct UnknownStepError {
    /// The raw string that wasn't a recongized step.
    pub unknown: String,
}
impl Display for UnknownStepError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult { write!(f, "Unknown preprocess step '{}'", self.unknown) }
}
impl Error for UnknownStepError {}





/***** LIBRARY *****/
/// Overview of all preprocess steps in the libary.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Step {
    /// The pruning of the graph as proposed by [1].
    Peek,
}
impl Step {
    /// Returns all implemented steps.
    ///
    /// # Returns
    /// A static list of the implemented steps.
    #[inline]
    pub const fn all() -> &'static [Self] { &[Self::Peek] }
}
impl FromStr for Step {
    type Err = UnknownStepError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "peek" => Ok(Self::Peek),
            other => Err(UnknownStepError { unknown: other.into() }),
        }
    }
}





/***** LIBRARY *****/
/// Defines a graph preprocessing step for K-Shortest Path algorithms.
pub trait PreprocessStep {
    /// Preprocesses a graph before applying K-Shortest Path to it.
    ///
    /// # Arguments
    /// - `graph`: The [`Graph`] to find in.
    /// - `src`: The source node to find a path from.
    /// - `dst`: The destination node to find a path to.
    /// - `k`: The number of paths to find.
    ///
    /// # Panics
    /// This function is allowed to panic if the given `src` or `dst` are not in the given `graph`.
    fn preprocess(graph: &mut Graph, src: &str, dst: &str, k: usize);
}
