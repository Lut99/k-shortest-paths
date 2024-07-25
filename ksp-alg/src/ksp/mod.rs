//  MOD.rs
//    by Lut99
//
//  Created:
//    24 Jul 2024, 01:44:45
//  Last edited:
//    26 Jul 2024, 01:23:28
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the various K-shortest Path algorithms.
//

// Define the algs
pub mod wikipedia;
pub mod yen;

// Imports
use ksp_graph::Graph;

use crate::sssp::Sssp;
use crate::utils::parsable_enum_impl;
use crate::Path;


/***** LIBRARY *****/
parsable_enum_impl! {
    /// Overview of all K-Shortest path algorithms in the libary.
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Ksp {
        Wikipedia { "wikipedia" => Self::Wikipedia },
        Yen(Sssp) { "yen<dijksta>" => Self::Yen(Sssp::Dijkstra) },
    }
}



/// Defines an abstraction over algorithms that compute the K shortest paths between two nodes in a
/// graph.
pub trait MultiRouting {
    /// Finds the K shortest paths from one node to another.
    ///
    /// # Arguments
    /// - `graph`: The [`Graph`] to find in.
    /// - `src`: The source node to find a path from.
    /// - `dst`: The destination node to find a path to.
    /// - `k`: The number of paths to find.
    ///
    /// # Returns
    /// A list of the `k` shortest paths found.
    ///
    /// # Panics
    /// This function is allowed to panic if the given `src` or `dst` are not in the given `graph` or they are not connected.
    fn k_shortest<'g>(graph: &'g Graph, src: &str, dst: &str, k: usize) -> Vec<Path<'g>>;
}
