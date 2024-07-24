//  MOD.rs
//    by Lut99
//
//  Created:
//    25 Jul 2024, 01:12:33
//  Last edited:
//    25 Jul 2024, 01:17:52
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines graph colouring algorithms.
//

use std::collections::HashMap;

use ksp_graph::Graph;


/***** AUXILLARY *****/
/// Defines a colouring of a [`Graph`].
#[derive(Clone, Debug)]
pub struct GraphColouring<'g> {
    /// The colours of nodes.
    pub nodes: HashMap<&'g str, Colour>,
    /// The colours of edges.
    pub edges: HashMap<&'g str, Colour>,
}

/// Defines the possible colours assigned to a graph node/edge.
#[derive(Clone, Debug)]
pub enum Colour {
    // Atomic
    /// It's a number.
    Number(f64),
    /// It's a string value.
    String(String),

    // Composite
    /// It's a tuple of multiple colours.
    Tuple(Vec<Self>),
}





/***** LIBRARY *****/
/// Defines an abstraction over graph colouring algorithms.
pub trait Colouring {
    /// Colours the graph in a particular way.
    ///
    /// # Arguments
    /// - `graph`: The [`Graph`] to colour.
    /// - `src`: The source node to find a path from.
    /// - `dst`: The destination node to find a path to.
    ///
    /// # Returns
    /// The shortest paths found.
    ///
    /// # Panics
    /// This function is allowed to panic if the given `src` or `dst` are not in the given `graph` or they are not connected.
    fn color<'g>(&mut self, graph: &'g Graph, src: &str, dst: &str) -> GraphColouring<'g>;
}
