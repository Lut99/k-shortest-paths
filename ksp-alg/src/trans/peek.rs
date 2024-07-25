//  PEEK.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 20:42:17
//  Last edited:
//    26 Jul 2024, 00:37:20
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the optimized `PeeK`-algorithm.
//!
//!   See [`peek()`] for more information.
//

use core::f64;
use std::collections::HashMap;
use std::marker::PhantomData;

use arrayvec::ArrayString;
use ksp_graph::Graph;

use super::Transforming;
use crate::dist::Distancing;
use crate::path::Path;


/***** TESTS *****/
#[cfg(test)]
mod tests {
    use super::*;
    use crate::dist::dijkstra::Dijkstra;
    use crate::utils::load_graph;

    #[test]
    fn test_peek_transform() {
        // Load the before- and after graph
        let mut g1: Graph = load_graph("peek_fig1a");
        let g2: Graph = load_graph("peek_fig1c");

        // Check whether it makes sense
        PeeK::<Dijkstra>::transform(&mut g1, "s", "t", 3);
        assert!(g1.same_structure(&g2));
    }
}





/***** HELPER FUNCTIONS *****/
/// Determines if a path is valid.
///
/// # Arguments
/// - `graph`: The graph to check the path in.
/// - `src_costs`: The costs of every node to the `src`-node.
/// - `dst_costs`: The costs of every node to the `dst`-node.
/// - `src`: The starting node.
/// - `dst`: The ending node.
/// - `node`: Some node through which the path from src to dst goes.
///
/// # Returns
/// True if it is, or false if it isn't.
fn is_valid(graph: &Graph, src_costs: &HashMap<&str, f64>, dst_costs: &HashMap<&str, f64>, src: &str, dst: &str, node: &str) -> bool {
    /// Builds a path between two nodes by following the coloration of the graph.
    ///
    /// # Arguments
    /// - `graph`: The graph to check the path in.
    /// - `costs`: The costs of every node to the `dst`-node.
    /// - `src`: The starting node.
    /// - `dst`: The ending node.
    ///
    /// # Returns
    /// The path taken.
    fn follow_path<'g>(graph: &'g Graph, costs: &HashMap<&str, f64>, src: &str, dst: &str) -> Path<'g> {
        let mut path: Path<'g> = Path { hops: vec![(graph.nodes.get_key_value(src).unwrap().0, 0.0)] };
        while path.end().unwrap() != dst {
            let current: &'g str = path.end().unwrap();

            // Get the next hop
            let mut best_hop: Option<(&'g str, f64)> = None;
            for edge in graph.edges.values() {
                // Get the neighbour
                let neigh: &str = if edge.left.as_str() == current && edge.right.as_str() != current {
                    edge.right.as_str()
                } else {
                    continue;
                };

                // See if it's better
                let ncost: f64 = *costs.get(neigh).unwrap();
                if ncost < best_hop.map(|(_, cost)| cost).unwrap_or(f64::INFINITY) {
                    best_hop = Some((neigh, ncost));
                }
            }

            // Check if there is a next hop
            match best_hop {
                Some((next, cost)) => path.hops.push((next, cost)),
                None => panic!("No path to '{dst}' from '{current}'"),
            }
        }

        // Done
        path
    }


    // Build a path from two components
    let node_src: Path = follow_path(graph, src_costs, node, src);
    let node_dst: Path = follow_path(graph, dst_costs, node, dst);

    // Double-check there's no duplicates
    for hop in &node_src.hops {
        if node_dst.hops.iter().find(|(n, _)| hop.0 == *n).is_some() {
            return false;
        }
    }
    true
}





/***** LIBRARY *****/
/// Defines a prune-centric approach for K-Shortest Path Computation (i.e., it be faster).
///
/// Based on \[1\].
///
/// # References
/// \[1\] W. Feng, S. Chen, H. Liu and Y. Ji, "Peek: A Prune-Centric Approach for K Shortest Path
///       Computation," in SC23: International Conference for High Performance Co doi:
///       10.1145/3581784.3607110 keywords: {runtime;upper bound;social networking (online);
///       instruction sets;image edge detection;high performance computing;blogs} url:
///       https://doi.ieeecomputersociety.org/10.1145/3581784.3607110
#[derive(Clone, Copy, Debug)]
pub struct PeeK<D> {
    _dist: PhantomData<D>,
}
impl<D: Distancing> Transforming for PeeK<D> {
    fn transform(graph: &mut Graph, src: &str, dst: &str, k: usize) {
        #[cfg(feature = "log")]
        log::debug!("Before PeeK: {} nodes, {} edges", graph.nodes.len(), graph.edges.len());

        // Step 1: Color the nodes by distance to source & distination
        let mut colours: HashMap<ArrayString<64>, f64> = HashMap::with_capacity(graph.nodes.len());
        let mut colours_order: Vec<ArrayString<64>> = Vec::with_capacity(graph.nodes.len());
        let b: f64 = {
            let src_colours: HashMap<&str, f64> = D::shortest_all(graph, src);
            let dst_colours: HashMap<&str, f64> = D::shortest_all(graph, dst);

            // Step 2: Identify the K-upper bound (b)
            for node in graph.nodes.values() {
                colours.insert(node.id.clone(), src_colours.get(node.id.as_str()).unwrap() + dst_colours.get(node.id.as_str()).unwrap());
                colours_order.push(node.id.clone());
            }
            colours_order.sort_by(|lhs, rhs| colours.get(lhs).unwrap().total_cmp(colours.get(rhs).unwrap()));
            let mut b: f64 = colours_order.last().map(|id| *colours.get(id).unwrap()).unwrap_or_default();
            let mut q_len: usize = 0;
            for (node, dist) in &colours {
                if is_valid(graph, &src_colours, &dst_colours, src, dst, node) {
                    q_len += 1;
                    if q_len == k {
                        b = *dist;
                        break;
                    }
                }
            }
            b
        };

        // Step 3: Prune any unnecessary nodes & edges
        graph.nodes.retain(|_, n| *colours.iter().find(|(i, _)| &n.id == *i).unwrap().1 <= b);
        graph.edges.retain(|_, e| e.cost <= b);

        // Step 4: Done!
        #[cfg(feature = "log")]
        log::debug!("After PeeK: {} nodes, {} edges", graph.nodes.len(), graph.edges.len());
    }
}
