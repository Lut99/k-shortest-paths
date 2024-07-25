//  PEEK.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 20:42:17
//  Last edited:
//    25 Jul 2024, 22:05:41
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the optimized `PeeK`-algorithm.
//!
//!   See [`peek()`] for more information.
//

use std::collections::HashMap;
use std::marker::PhantomData;

use arrayvec::ArrayString;
use ksp_graph::Graph;

use super::Transforming;
use crate::dist::Distancing;


/***** TESTS *****/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek_transform() {}
}





/***** HELPER FUNCTIONS *****/
/// Determines if a path is valid.
///
/// # Arguments
/// - ``
///
/// # Returns
/// True if it is, or false if it isn't.
fn is_valid() -> bool { true }





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
pub struct Peek<D> {
    _dist: PhantomData<D>,
}
impl<D: Distancing> Transforming for Peek<D> {
    fn transform(graph: &mut Graph, src: &str, dst: &str, k: usize) {
        #[cfg(feature = "log")]
        log::debug!("Before PeeK: {} nodes, {} edges", graph.nodes.len(), graph.edges.len());

        // Step 1: Color the nodes by distance to source & distination
        let mut colours: Vec<(ArrayString<64>, f64)> = Vec::with_capacity(graph.nodes.len());
        let b: f64 = {
            let src_colours: HashMap<&str, f64> = D::shortest_all(graph, src);
            let dst_colours: HashMap<&str, f64> = D::shortest_all(graph, dst);

            // Step 2: Identify the K-upper bound (b)
            for node in graph.nodes.values() {
                colours.push((node.id.clone(), src_colours.get(node.id.as_str()).unwrap() + dst_colours.get(node.id.as_str()).unwrap()));
            }
            colours.sort_by(|lhs, rhs| lhs.1.total_cmp(&rhs.1));
            let mut b: f64 = colours.last().map(|(_, dist)| *dist).unwrap_or_default();
            let mut q_len: usize = 0;
            for (node, dist) in &colours {
                if is_valid() {
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
        graph.nodes.retain(|_, n| colours.iter().find(|(i, _)| &n.id == i).unwrap().1 <= b);
        graph.edges.retain(|_, e| e.cost <= b);

        // Step 4: Done!
        #[cfg(feature = "log")]
        log::debug!("After PeeK: {} nodes, {} edges", graph.nodes.len(), graph.edges.len());
    }
}
