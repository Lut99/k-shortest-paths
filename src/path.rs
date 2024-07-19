//  PATH.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 02:05:23
//  Last edited:
//    20 Jul 2024, 01:54:54
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines a path between two nodes.
//

use std::fmt::{Display, Formatter, Result as FResult};
use std::hash::{Hash, Hasher};


/***** LIBRARY *****/
/// Convenience macro for building paths with auto-computed cost.
///
/// # Panics
/// The produced code will generate an error if the given path does not exist with direct links in the given graph.
#[macro_export]
macro_rules! path {
    // Counting of path nodes
    (__COUNT) => { 0 };
    (__COUNT $node:literal) => { 1 };
    (__COUNT $node:literal $($nodes:literal)+) => { 1 + ::ksp::path!(__COUNT $($nodes)+) };
    // Counting of path nodes (local)
    (__COUNT crate:) => { 0 };
    (__COUNT crate: $node:literal) => { 1 };
    (__COUNT crate: $node:literal $($nodes:literal)+) => { 1 + crate::path!(__COUNT $crt $($nodes)+) };

    // Main interface
    ($graph:expr, $start:literal -> $end:literal) => {
        {
            // Build the path components
            let graph = &$graph;
            let mut cost: f64 = 0.0;
            let mut hops: Vec<(&'static str, f64)> = Vec::with_capacity(1 + ::ksp::path!(__COUNT :) + 1);
            hops.push(($start, cost));
            'hops: for (left, right) in [$start].into_iter().zip([$end]) {
                // Find an edge from left-to-right
                for edge in graph.edges.values() {
                    if (edge.left.as_str() == left && edge.right.as_str() == right) || (edge.left.as_str() == right && edge.right.as_str() == left) {
                        cost += edge.cost;
                        hops.push((right, cost));
                        break 'hops;
                    }
                }
                panic!("There is no link between nodes {left:?} and {right:?}");
            }
            ::ksp::path::Path { hops }
        }
    };
    (crate : $graph:expr, $start:literal $(-> $nodes:literal)* -| $end:literal) => {
        {
            // Build the path components
            let graph = &$graph;
            let mut cost: f64 = 0.0;
            let mut hops: Vec<(&'static str, f64)> = Vec::with_capacity(1 + crate::path!(__COUNT crate :) + 1);
            hops.push(($start, cost));
            'hops: for (left, right) in [$start $(,$nodes)*].into_iter().zip([$($nodes,)* $end]) {
                // Find an edge from left-to-right
                for edge in graph.edges.values() {
                    if (edge.left.as_str() == left && edge.right.as_str() == right) || (edge.left.as_str() == right && edge.right.as_str() == left) {
                        cost += edge.cost;
                        hops.push((right, cost));
                        continue 'hops;
                    }
                }
                panic!("There is no link between nodes {left:?} and {right:?}");
            }
            crate::path::Path { hops }
        }
    };
}



/// Defines a path between two nodes.
#[derive(Clone, Debug)]
pub struct Path<'g> {
    /// The hops of the path.
    pub hops: Vec<(&'g str, f64)>,
}
impl<'g> Path<'g> {
    /// Returns the endpoint of this path, if any.
    ///
    /// # Returns
    /// A reference to the ID of the endpoint of the path.
    #[inline]
    pub fn end(&self) -> Option<&'g str> { self.hops.last().map(|(n, _)| *n) }

    /// Returns the cost of this path.
    ///
    /// # Returns
    /// The cost of the entire path.
    #[inline]
    pub fn cost(&self) -> f64 { self.hops.last().map(|(_, c)| *c).unwrap_or(0.0) }
}

impl<'g> Display for Path<'g> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FResult {
        let mut first: bool = true;
        for (node, cost) in &self.hops {
            if first {
                first = false;
            } else {
                write!(f, " -{cost}-> ")?;
            }
            write!(f, "{node}")?;
        }
        Ok(())
    }
}

impl<'g> Eq for Path<'g> {}
impl<'g> Hash for Path<'g> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (hop, _) in &self.hops {
            hop.hash(state)
        }
    }
}
impl<'g> PartialEq for Path<'g> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.hops.len() != other.hops.len() {
            return false;
        }
        for i in 0..self.hops.len() {
            if self.hops[i].0 != other.hops[i].0 {
                return false;
            }
        }
        return true;
    }
}
