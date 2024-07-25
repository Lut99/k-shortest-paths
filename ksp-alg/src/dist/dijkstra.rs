//  DIJKSTRA.rs
//    by Lut99
//
//  Created:
//    24 Jul 2024, 00:43:39
//  Last edited:
//    25 Jul 2024, 22:06:13
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements Dijkstra's SSSP algorithm but as a colouring procedure.
//

use std::collections::HashMap;

use ksp_graph::Graph;

use super::Distancing;


/***** TESTS *****/
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::load_graph;

    #[test]
    fn test_dijkstra_dist() {
        // Run it quite some times to catch hashmap problems
        for _ in 0..10 {
            let g: Graph = load_graph("cities");
            assert_eq!(
                Dijkstra::shortest_all(&g, "Amsterdam"),
                HashMap::from([("Amsterdam", 0.0), ("Berlin", 577.34), ("Chicago", 540.86 + 6249.15), ("Dorchester", 540.86), ("Edinburgh", 660.68)])
            );
            assert_eq!(
                Dijkstra::shortest_all(&g, "Berlin"),
                HashMap::from([
                    ("Amsterdam", 577.34),
                    ("Berlin", 0.0),
                    ("Chicago", 577.34 + 540.86 + 6249.15),
                    ("Dorchester", 577.34 + 540.86),
                    ("Edinburgh", 577.34 + 660.68)
                ])
            );
            assert_eq!(
                Dijkstra::shortest_all(&g, "Chicago"),
                HashMap::from([
                    ("Amsterdam", 6249.15 + 540.86),
                    ("Berlin", 6249.15 + 540.86 + 577.34),
                    ("Chicago", 0.0),
                    ("Dorchester", 6249.15),
                    ("Edinburgh", 6249.15 + 589.23)
                ])
            );
            assert_eq!(
                Dijkstra::shortest_all(&g, "Dorchester"),
                HashMap::from([("Amsterdam", 540.86), ("Berlin", 540.86 + 577.34), ("Chicago", 6249.15), ("Dorchester", 0.0), ("Edinburgh", 589.23)])
            );
            assert_eq!(
                Dijkstra::shortest_all(&g, "Edinburgh"),
                HashMap::from([
                    ("Amsterdam", 660.68),
                    ("Berlin", 660.68 + 577.34),
                    ("Chicago", 589.23 + 6249.15),
                    ("Dorchester", 589.23),
                    ("Edinburgh", 0.0)
                ])
            );
        }
    }
}





/***** LIBRARY *****/
/// Defines the Dijkstra's A* algorithm \[2\] as a coloring algorithm for the entire graph.
///
/// Specifically, annotates every node with the cost of the shortest path to some target node.
///
/// # References
/// \[2\] Dijkstra, E.W. A note on two problems in connexion with graphs.
/// _Numer. Math._ 1, 269–271 (1959). https://doi.org/10.1007/BF01386390.
pub struct Dijkstra;
impl Distancing for Dijkstra {
    #[track_caller]
    fn shortest_all<'g>(graph: &'g Graph, dst: &str) -> HashMap<&'g str, f64> {
        // Do a depth-first search with the shortest path heuristic
        let mut distances: HashMap<&'g str, (f64, bool)> =
            graph.nodes.keys().map(|id| (id.as_str(), if id.as_str() == dst { (0.0, false) } else { (f64::INFINITY, false) })).collect();

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

            // Update all distances
            for edge in graph.edges.values() {
                // Get the neighbour of this node
                let neigh: &str = if edge.left.as_str() == next && edge.right.as_str() != next {
                    edge.right.as_str()
                // } else if edge.left.as_str() != next && edge.right.as_str() == next {
                //     edge.left.as_str()
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

        // OK, done
        distances.into_iter().map(|(k, (d, _))| (k, d)).collect()
    }
}
