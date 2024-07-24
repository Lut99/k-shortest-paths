//  DIJKSTRA.rs
//    by Lut99
//
//  Created:
//    24 Jul 2024, 00:43:39
//  Last edited:
//    24 Jul 2024, 20:41:16
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements Dijkstra's SSSP algorithm.
//

use std::collections::HashMap;

use ksp_graph::Graph;

use super::SingleShortestPath;
use crate::path::Path;


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
            assert_eq!(DijkstraSSSP.shortest(&g, "Amsterdam", "Berlin"), path!(crate : g, "Amsterdam" -| "Berlin"));
            assert_eq!(DijkstraSSSP.shortest(&g, "Amsterdam", "Dorchester"), path!(crate : g, "Amsterdam" -| "Dorchester"));
            assert_eq!(DijkstraSSSP.shortest(&g, "Amsterdam", "Chicago"), path!(crate : g, "Amsterdam" -> "Dorchester" -| "Chicago"));
            assert_eq!(DijkstraSSSP.shortest(&g, "Berlin", "Chicago"), path!(crate : g, "Berlin" -> "Amsterdam" -> "Dorchester" -| "Chicago"));
        }
    }
}





/***** LIBRARY *****/
/// Defines the SSSP (Single-Source Shortest Path) used in Yen's algorithm.
///
/// In particular, we implement Dijkstra's SSSP Algorithm \[2\], a.k.a., A*.
///
/// # References
/// \[2\] Dijkstra, E.W. A note on two problems in connexion with graphs.
/// _Numer. Math._ 1, 269–271 (1959). https://doi.org/10.1007/BF01386390.
pub struct DijkstraSSSP;
impl SingleShortestPath for DijkstraSSSP {
    #[track_caller]
    fn shortest<'g>(&mut self, graph: &'g Graph, src: &str, dst: &str) -> Path<'g> {
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
}
