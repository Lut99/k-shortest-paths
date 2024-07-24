//  LIB.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 00:06:19
//  Last edited:
//    24 Jul 2024, 02:14:29
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the different implementations of K-Shortest Path (KSP)
//!   algorithms.
//

// Declare modules
pub mod ksp;
pub mod path;
pub mod prep;
pub mod sssp;
#[cfg(test)]
pub mod utils;

use std::time::Duration;

// Use some of it in this namespace
pub use crate::ksp::*;
pub use crate::path::*;


/***** ERRORS *****/
/// Failed to parse a [`Pipeline`] from a string.
#[derive(Debug)]
pub enum PipelineParseError {}





/***** HELPERS *****/
/// Defines profile timings of a [`Pipeline`]-run.
#[derive(Clone, Debug)]
pub struct PipelineProfile {
    /// The amount of time each step took.
    prep: Vec<Duration>,
    /// The time the main algorithm took.
    alg:  Duration,
    /// The timings for all SSSP calls, if any.
    sssp: Vec<Duration>,
}





/***** LIBRARY *****/
/// Defines a full chain that configures which KSP algorithm is run and how.
#[derive(Clone, Debug)]
pub struct Pipeline {
    /// Preprocess steps to take.
    prep: Vec<prep::Step>,
    /// The algorithm to execute.
    alg:  Algorithm,
    /// Which SSSP algorithm to use if applicable.
    sssp: Option<sssp::Sssp>,
}
impl Pipeline {
    /// Computes the K-Shortest Path algorithm as defined by this [`Pipeline`].
    ///
    /// # Arguments
    /// - `graph`: The [`Graph`] to find in.
    /// - `src`: The source node to find a path from.
    /// - `dst`: The destination node to find a path to.
    /// - `k`: The number of paths to find.
    ///
    /// # Returns
    /// A pair of the list of the shortest paths found and a [`PipelineProfile`] detailling how long every step took.
    ///
    /// The path list is at most `k` elements long.
    ///
    /// # Panics
    /// This function is allowed to panic if the given `src` or `dst` are not in the given `graph`.
    #[inline]
    pub fn k_shortest_paths_profiled<'g>(&self, graph: &'g mut ksp_graph::Graph, src: &str, dst: &str, k: usize) -> (Vec<Path<'g>>, PipelineProfile) {
        // First, pre-process the graph
        for p in &self.prep {
            use prep::PreprocessStep as _;
            match p {
                prep::Step::Peek => prep::peek::PeekPreprocess::preprocess(graph, src, dst, k),
            }
        }

        // Run the appropriate KSP algorithm
        match (&self.alg, &self.sssp) {
            (Algorithm::Wikipedia, _) => ksp::wikipedia::WikipediaKSP::k_shortest_paths(graph, src, dst, k),
            (Algorithm::Yen, Some(sssp::Sssp::Dijkstra)) => ksp::yen::YenKSP::<sssp::dijkstra::DijkstraSSSP>::k_shortest_paths(graph, src, dst, k),
            (Algorithm::Yen, None) => panic!("Cannot run Yen without SSSP defined"),
        }
    }
}
impl std::str::FromStr for Pipeline {
    type Err = PipelineParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!();
    }
}
