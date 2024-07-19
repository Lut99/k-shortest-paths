//  VANILLA.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 00:10:52
//  Last edited:
//    20 Jul 2024, 01:54:36
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the simplest KSP algorithm as presented by the PeeK-paper [1].
//!   
//!   See the [`peek`](super::peek) module for the reference.
//

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use arrayvec::ArrayString;
use ksp_graph::Graph;

use crate::path::Path;
use crate::Routing;


/***** TESTS *****/
#[cfg(test)]
mod tests {
    use super::*;
    use crate::path;
    use crate::utils::load_graph;

    #[test]
    fn test_sssp() {
        // Run it quite some times to catch hashmap problems
        for _ in 0..10 {
            let g: Graph = load_graph("cities");
            assert_eq!(sssp(&g, "Amsterdam", "Berlin"), path!(crate : g, "Amsterdam" -| "Berlin"));
            assert_eq!(sssp(&g, "Amsterdam", "Dorchester"), path!(crate : g, "Amsterdam" -| "Dorchester"));
            assert_eq!(sssp(&g, "Amsterdam", "Chicago"), path!(crate : g, "Amsterdam" -> "Dorchester" -| "Chicago"));
            assert_eq!(sssp(&g, "Berlin", "Chicago"), path!(crate : g, "Berlin" -> "Amsterdam" -> "Dorchester" -| "Chicago"));
        }
    }

    #[test]
    fn test_yen_ksp() {
        // Run it quite some times to catch hashmap problems
        for _ in 0..10 {
            let g: Graph = load_graph("cities");
            assert_eq!(YenKSP.k_shortest_paths(&g, "Amsterdam", "Berlin", 1), vec![path!(crate : g, "Amsterdam" -| "Berlin")]);
            assert_eq!(YenKSP.k_shortest_paths(&g, "Amsterdam", "Dorchester", 1), vec![path!(crate : g, "Amsterdam" -| "Dorchester")]);
            assert_eq!(YenKSP.k_shortest_paths(&g, "Amsterdam", "Chicago", 1), vec![path!(crate : g, "Amsterdam" -> "Dorchester" -| "Chicago")]);
            assert_eq!(YenKSP.k_shortest_paths(&g, "Berlin", "Chicago", 1), vec![
                path!(crate : g, "Berlin" -> "Amsterdam" -> "Dorchester" -| "Chicago")
            ]);
        }
    }
}





/***** AUXILLARY *****/
/// Defines the SSSP (Single-Source Shortest Path) used in Yen's algorithm.
///
/// In particular, we implement Dijkstra's SSSP Algorithm \[2\], a.k.a., A*.
///
/// # Arguments
/// - `graph`: The [`Graph`] to find in.
/// - `src`: The source node to find a path from.
/// - `dst`: The destination node to find a path to.
///
/// # Returns
/// A new [`Path`] that represents the shortest between `src` and `dst`.
///
/// # References
/// \[2\] Dijkstra, E.W. A note on two problems in connexion with graphs.
/// _Numer. Math._ 1, 269–271 (1959). https://doi.org/10.1007/BF01386390.
#[track_caller]
pub fn sssp<'g>(graph: &'g Graph, src: &str, dst: &str) -> Path<'g> {
    // Do a depth-first search with the shortest path heuristic
    let mut distances: HashMap<&'g str, (f64, bool)> =
        graph.nodes.keys().map(|id| (id.as_str(), if id.as_str() == src { (0.0, false) } else { (f64::INFINITY, false) })).collect();

    // Loop to populate the distances
    loop {
        // Find the node to treat
        let mut next: Option<(&'g str, f64)> = None;
        for (node, (distance, visited)) in &distances {
            if !visited && *distance < next.map(|(_, d)| d).unwrap_or(f64::INFINITY) {
                next = Some((node, *distance));
            }
        }
        let (next, cost): (&'g str, f64) = match next {
            Some(next) => next,
            None => break,
        };
        if next == dst {
            break;
        }

        // Update all distances
        for edge in graph.edges.values() {
            // Get the neighbour of this node
            let neigh: &str = if edge.left.as_str() == next && edge.right.as_str() != next {
                edge.right.as_str()
            } else if edge.left.as_str() != next && edge.right.as_str() == next {
                edge.left.as_str()
            } else {
                continue;
            };

            // Update its value, but only iff shorter
            let neigh_dist: &mut f64 = &mut distances.get_mut(neigh).unwrap().0;
            if cost + edge.cost < *neigh_dist {
                *neigh_dist = cost + edge.cost;
            }
        }

        // Mark this node as visited
        distances.get_mut(next).unwrap().1 = true;
    }

    // To find the path, now walk it backwards
    let dst_dist: (&&'g str, &(f64, bool)) = distances.get_key_value(dst).unwrap();
    let mut path: Path<'g> = Path { hops: vec![(dst_dist.0, dst_dist.1.0)] };
    while path.hops[0].0 != src {
        // Get the next edge leading to the smallest distance
        let mut nearest: Option<(&'g str, f64)> = None;
        for edge in graph.edges.values() {
            // Get the neighbour of this node
            let neigh: &str = if edge.left.as_str() == path.hops[0].0 && edge.right.as_str() != path.hops[0].0 {
                edge.right.as_str()
            } else if edge.left.as_str() != path.hops[0].0 && edge.right.as_str() == path.hops[0].0 {
                edge.left.as_str()
            } else {
                continue;
            };

            // Store it only if the smallest
            let dist: f64 = distances.get(neigh).unwrap().0;
            if dist < nearest.map(|(_, d)| d).unwrap_or(f64::INFINITY) {
                nearest = Some((neigh, dist));
            }
        }
        match nearest {
            Some((node, cost)) => path.hops.insert(0, (node, cost)),
            None => panic!("Source '{src}' and destination '{dst}' nodes are not connected"),
        }
    }
    path
}





/***** LIBRARY *****/
/// Defines the vanilla, simplest version of a KSP-algorithm.
///
/// Based on: <https://en.wikipedia.org/wiki/K_shortest_path_routing#Algorithm>
#[derive(Clone, Copy, Debug)]
pub struct YenKSP;
impl Routing for YenKSP {
    #[track_caller]
    fn k_shortest_paths<'g>(&mut self, graph: &'g Graph, src: &str, dst: &str, k: usize) -> Vec<Path<'g>> {
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
        shortest.push(sssp(graph, src, dst));
        let mut candidates: HashSet<Path<'g>> = HashSet::with_capacity(k);
        for i in 1..k {
            // Consider the shortest paths of this length
            // candidates.clear();
            for hop in 0..shortest[i - 1].hops.len() {
                let prefix: &[(&'g str, f64)] = &shortest[i - 1].hops[..i];
                let suffix: Path<'g> = sssp(graph, shortest[i - 1].hops[hop].0, dst);
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
