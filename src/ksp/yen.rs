//  VANILLA.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 00:10:52
//  Last edited:
//    24 Jul 2024, 02:04:26
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the simplest KSP algorithm as presented by the PeeK-paper [1].
//!   
//!   See the [`peek`](super::peek) module for the reference.
//

use std::cmp::Ordering;
use std::collections::HashSet;
use std::marker::PhantomData;

use arrayvec::ArrayString;
use ksp_graph::Graph;

use super::KShortestPath;
use crate::path::Path;
use crate::sssp::SingleShortestPath;


/***** TESTS *****/
#[cfg(test)]
mod tests {
    use super::*;
    use crate::path;
    use crate::sssp::dijkstra::DijkstraSSSP;
    use crate::utils::load_graph;

    #[test]
    fn test_yen_ksp() {
        // Run it quite some times to catch hashmap problems
        for _ in 0..10 {
            let g: Graph = load_graph("cities");
            assert_eq!(YenKSP::<DijkstraSSSP>::k_shortest_paths(&g, "Amsterdam", "Berlin", 1), vec![path!(crate : g, "Amsterdam" -| "Berlin")]);
            assert_eq!(YenKSP::<DijkstraSSSP>::k_shortest_paths(&g, "Amsterdam", "Dorchester", 1), vec![
                path!(crate : g, "Amsterdam" -| "Dorchester")
            ]);
            assert_eq!(YenKSP::<DijkstraSSSP>::k_shortest_paths(&g, "Amsterdam", "Chicago", 1), vec![
                path!(crate : g, "Amsterdam" -> "Dorchester" -| "Chicago")
            ]);
            assert_eq!(YenKSP::<DijkstraSSSP>::k_shortest_paths(&g, "Berlin", "Chicago", 1), vec![
                path!(crate : g, "Berlin" -> "Amsterdam" -> "Dorchester" -| "Chicago")
            ]);
        }
    }
}





/***** LIBRARY *****/
/// Defines the vanilla, simplest version of a KSP-algorithm.
///
/// Based on: <https://en.wikipedia.org/wiki/K_shortest_path_routing#Algorithm>
#[derive(Clone, Copy, Debug)]
pub struct YenKSP<S> {
    /// The SSSP algorithm used.
    _sssp: PhantomData<S>,
}
impl<S: SingleShortestPath> KShortestPath for YenKSP<S> {
    #[track_caller]
    fn k_shortest_paths<'g>(graph: &'g Graph, src: &str, dst: &str, k: usize) -> Vec<Path<'g>> {
        // Assert that both nodes exists
        let src: &'g str = if let Some((key, _)) = graph.nodes.get_key_value(&ArrayString::from(src).unwrap()) {
            key
        } else {
            panic!("Unknown source node '{src}'");
        };
        if !graph.nodes.contains_key(&ArrayString::from(dst).unwrap()) {
            panic!("Unknown source node '{dst}'");
        }

        // Then do the algorithm
        let mut shortest: Vec<Path<'g>> = Vec::with_capacity(k);
        shortest.push(S::shortest(graph, src, dst));
        let mut candidates: HashSet<Path<'g>> = HashSet::with_capacity(k);
        for i in 1..k {
            // Consider the shortest paths of this length
            // candidates.clear();
            for hop in 0..shortest[i - 1].hops.len() {
                let prefix: &[(&'g str, f64)] = &shortest[i - 1].hops[..i];
                let suffix: Path<'g> = S::shortest(graph, shortest[i - 1].hops[hop].0, dst);
                let path: Path<'g> = Path {
                    hops: prefix.into_iter().copied().chain(suffix.hops.into_iter().map(|(n, c)| (n, prefix.last().unwrap().1 + c))).collect(),
                };
                candidates.insert(path);
            }

            // Store it
            if let Some(min) = candidates.iter().min_by(|p1, p2| p1.cost().partial_cmp(&p2.cost()).unwrap_or(Ordering::Equal)) {
                shortest.push(min.clone());
            }
        }

        // OK, done
        shortest
    }
}
