//  PEEK.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 20:42:17
//  Last edited:
//    24 Jul 2024, 02:05:51
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

use super::PreprocessStep;
use crate::path::Path;
use crate::sssp::dijkstra::DijkstraSSSP;


/***** LIBRARY *****/
/// Defines a prune-centric approach for K-Shortest Path Computation (i.e., it be faster).
///
/// Based on \[1\].
#[derive(Clone, Copy, Debug)]
pub struct PeekPreprocess;
impl PreprocessStep for PeekPreprocess {
    fn preprocess(graph: &mut Graph, src: &str, dst: &str, k: usize) {
        todo!();
    }
}
