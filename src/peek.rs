//  PEEK.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 20:42:17
//  Last edited:
//    19 Jul 2024, 23:47:00
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the optimized `PeeK`-algorithm.
//!
//!   \[1\] W. Feng, S. Chen, H. Liu and Y. Ji, "Peek: A Prune-Centric Approach for K Shortest Path
//!       Computation," in SC23: International Conference for High Performance Co doi:
//!       10.1145/3581784.3607110 keywords: {runtime;upper bound;social networking (online);
//!       instruction sets;image edge detection;high performance computing;blogs} url:
//!       https://doi.ieeecomputersociety.org/10.1145/3581784.3607110
//

use ksp_graph::Graph;

use crate::path::Path;
use crate::Routing;


/***** LIBRARY *****/
/// Defines a prune-centric approach for K-Shortest Path Computation (i.e., it be faster).
///
/// Based on \[1\].
#[derive(Clone, Copy, Debug)]
pub struct PeekKSP;
impl Routing for PeekKSP {
    fn k_shortest_paths<'g>(&mut self, graph: &'g Graph, src: &str, dst: &str, k: usize) -> Vec<Path<'g>> {
        todo!();
    }
}
