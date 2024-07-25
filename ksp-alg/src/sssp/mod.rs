//  MOD.rs
//    by Lut99
//
//  Created:
//    24 Jul 2024, 00:41:28
//  Last edited:
//    26 Jul 2024, 01:23:32
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines a few Single-Source ShortestPath (SSSP) algorithms.
//

// Declarations
pub mod dijkstra;

// Imports
use ksp_graph::Graph;

use crate::path::Path;
use crate::utils::parsable_enum_impl;


/***** LIBRARY *****/
parsable_enum_impl! {
    /// Overview of all SSSP algorithms in the libary.
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Sssp {
        /// Arguably the most famous one from Dijkstra ([2]).
        Dijkstra { "dijkstra" => Self::Dijkstra },
    }
}



/// Defines an abstraction over algorithms that compute a single shortest path between two nodes in
/// a graph.
pub trait Routing {
    /// Finds the shortest paths from one node to another.
    ///
    /// # Arguments
    /// - `graph`: The [`Graph`] to find in.
    /// - `src`: The source node to find a path from.
    /// - `dst`: The destination node to find a path to.
    ///
    /// # Returns
    /// The shortest path found.
    ///
    /// # Panics
    /// This function is allowed to panic if the given `src` or `dst` are not in the given `graph` or they are not connected.
    fn shortest<'g>(graph: &'g Graph, src: &str, dst: &str) -> Path<'g>;
}
