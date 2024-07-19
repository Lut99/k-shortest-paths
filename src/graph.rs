//  GRAPH.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 00:21:34
//  Last edited:
//    19 Jul 2024, 01:01:52
//  Auto updated?
//    Yes
//
//  Description:
//!   Representation of the graphs that the KSP-algorithms search in.
//

use std::collections::HashMap;

use arrayvec::ArrayString;


/***** LIBRARY *****/
/// Defines a graph of nodes linked by edges.
#[derive(Clone, Debug)]
pub struct Graph {
    /// The nodes in the graph.
    pub nodes: HashMap<ArrayString<64>, Node>,
    /// The edges in the graph.
    pub edges: HashMap<ArrayString<64>, Edge>,
}

/// Defines a node in each graph.
#[derive(Clone, Copy, Debug)]
pub struct Node {
    /// The identifier of the node.
    pub id:  ArrayString<64>,
    /// If there's any coordinate information available, this will place it in a 2D-space.
    pub pos: (f64, f64),
}

/// Defines a link between nodes in each graph.
#[derive(Clone, Copy, Debug)]
pub struct Edge {
    /// The identifier of the edge.
    pub id:    ArrayString<64>,
    /// The ID of the first [`Node`] this edge connects.
    pub left:  ArrayString<64>,
    /// The ID of the second [`Node`] this edge connects.
    pub right: ArrayString<64>,
    /// The cost associated with traversing the edge.
    pub cost:  f64,
}
