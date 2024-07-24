//  PEEK.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 20:42:17
//  Last edited:
//    25 Jul 2024, 01:21:50
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the optimized `PeeK`-algorithm.
//!
//!   See [`peek()`] for more information.
//

use ksp_graph::Graph;


/***** LIBRARY *****/
/// Defines a prune-centric approach for K-Shortest Path Computation (i.e., it be faster).
///
/// Based on \[1\].
///
/// # Arguments
/// - `graph`: The graph to prune.
/// - `src`: The source node from which we will compute paths later.
/// - `dst`: The target node to which we will compute paths later.
/// - `k`: The number of paths to compute later.
///
/// # References
/// \[1\] W. Feng, S. Chen, H. Liu and Y. Ji, "Peek: A Prune-Centric Approach for K Shortest Path
///       Computation," in SC23: International Conference for High Performance Co doi:
///       10.1145/3581784.3607110 keywords: {runtime;upper bound;social networking (online);
///       instruction sets;image edge detection;high performance computing;blogs} url:
///       https://doi.ieeecomputersociety.org/10.1145/3581784.3607110
pub fn peek(graph: &mut Graph, src: &str, dst: &str, k: usize) {
    // Step 1: Color the nodes by distance to source & distination

    // Step 2: Identify
}
