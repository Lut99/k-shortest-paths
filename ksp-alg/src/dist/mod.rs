//  MOD.rs
//    by Lut99
//
//  Created:
//    25 Jul 2024, 20:21:42
//  Last edited:
//    26 Jul 2024, 01:23:37
//  Auto updated?
//    Yes
//
//  Description:
//!   Like [SSSP](crate::sssp), but then colouring algorithms for every
//!   node in the graph instead of finding a single path.
//

// Declare modules
pub mod dijkstra;

// Imports
use std::collections::HashMap;

use ksp_graph::Graph;

use crate::utils::parsable_enum_impl;


/***** LIBRARY *****/
parsable_enum_impl! {
    /// Overview of all distance colouring algorithms in the libary.
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Distance {
        /// Arguably the most famous one from Dijkstra (\[2\]).
        Dijkstra { "dijkstra" => Self::Dijkstra },
    }
}



/// Defines an abstraction over algorithms that compute the shortest path to a particular node, for
/// every node.
pub trait Distancing {
    /// Computes the shortest distance for every node to a particular target node.
    ///
    /// # Arguments
    /// - `graph`: The [`Graph`] to colour.
    /// - `dst`: The destination node to find distances to.
    ///
    /// # Returns
    /// An annotation of the distance to `dst` for every node in the given `graph`.
    ///
    /// Note that not all nodes in the `graph` can end up in the result. If omitted, it means there
    /// is no path from that node to `dst`.
    ///
    /// # Panics
    /// This function is allowed to panic if the given `src` or `dst` are not in the given `graph` or they are not connected.
    fn shortest_all<'g>(graph: &'g Graph, dst: &str) -> HashMap<&'g str, f64>;
}
