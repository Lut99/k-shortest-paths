//  MOD.rs
//    by Lut99
//
//  Created:
//    24 Jul 2024, 01:48:03
//  Last edited:
//    25 Jul 2024, 20:37:28
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines preprocessing steps for K-Shortest Path algorithms.
//

// Declare the modules
pub mod peek;

// Imports
use ksp_graph::Graph;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::dist::Distance;
use crate::utils::parsable_enum_impl;


/***** LIBRARY *****/
parsable_enum_impl! {
    /// Overview of all graph transforming algorithms in the libary.
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub enum Transformer {
        PeeK(Distance) { "peek<dijkstra>" => Self::PeeK(Distance::Dijkstra) },
    }
}



/// Defines an abstraction over algorithms that transform a graph.
pub trait Transforming {
    /// Takes a graph and transforms it as a way to preprocess it.
    ///
    /// # Arguments
    /// - `graph`: The [`Graph`] to transform. This will happen in-place.
    /// - `src`: The source node of any path we'd like to find soon.
    /// - `dst`: The destination node of any path we'd like to find soon.
    /// - `k`: The number of paths we would like to find soon.
    ///
    /// # Panics
    /// This function is allowed to panic if the given `src` or `dst` are not in the given `graph` or they are not connected.
    fn transform(graph: &mut Graph, src: &str, dst: &str, k: usize);
}
