//  VANILLA.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 00:10:52
//  Last edited:
//    23 Jul 2024, 01:52:18
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the simplest KSP algorithm as presented by Wikipedia.
//!   
//!   Based on: <https://en.wikipedia.org/wiki/K_shortest_path_routing#Algorithm>
//

use std::collections::{BTreeSet, HashMap};

use arrayvec::ArrayString;
use ksp_graph::Graph;

use crate::path::Path;
use crate::Routing;


/***** TESTS *****/
#[cfg(test)]
mod tests {
    use super::*;
    use crate::path;
    use crate::utils::{load_bench, load_graph};

    #[test]
    fn test_wikipedia_ksp_cities() {
        // Run it quite some times to catch hashmap problems
        for _ in 0..10 {
            let g: Graph = load_graph("cities");
            assert_eq!(WikipediaKSP.k_shortest_paths(&g, "Amsterdam", "Berlin", 1), vec![path!(crate : g, "Amsterdam" -| "Berlin")]);
            assert_eq!(WikipediaKSP.k_shortest_paths(&g, "Amsterdam", "Dorchester", 1), vec![path!(crate : g, "Amsterdam" -| "Dorchester")]);
            assert_eq!(WikipediaKSP.k_shortest_paths(&g, "Amsterdam", "Chicago", 1), vec![
                path!(crate : g, "Amsterdam" -> "Dorchester" -| "Chicago")
            ]);
            assert_eq!(WikipediaKSP.k_shortest_paths(&g, "Berlin", "Chicago", 1), vec![
                path!(crate : g, "Berlin" -> "Amsterdam" -> "Dorchester" -| "Chicago")
            ]);
        }
    }

    #[test]
    fn test_wikipedia_ksp_india35() {
        // Run some more difficult ones
        for _ in 0..10 {
            let g: Graph = load_bench("india35");
            assert_eq!(WikipediaKSP.k_shortest_paths(&g, "12", "33", 1), vec![path!(crate : g, "12" -| "33")]);
        }
    }
}





/***** LIBRARY *****/
/// Defines the vanilla, simplest version of a KSP-algorithm.
///
/// Based on: <https://en.wikipedia.org/wiki/K_shortest_path_routing#Algorithm>
#[derive(Clone, Copy, Debug)]
pub struct WikipediaKSP;
impl Routing for WikipediaKSP {
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
        let mut shortest_to: HashMap<&str, usize> = HashMap::with_capacity(graph.nodes.len());
        let mut todo: BTreeSet<Path<'g>> = BTreeSet::from([Path { hops: vec![(src, 0.0)] }]);
        while !todo.is_empty() && *shortest_to.entry(dst).or_default() < k {
            let path: Path<'g> = todo.pop_first().unwrap();
            let end: &str = path.end().unwrap();

            // Note how many paths we found to this node
            *shortest_to.entry(end).or_default() += 1;
            // Also mark it as shortest if the end is our destination
            if dst == end {
                shortest.push(path.clone());
            }

            // Next, we find next candidates
            if *shortest_to.get(end).unwrap() <= k {
                for e in graph.edges.values() {
                    // Find the next hope to take _if_ this is the correct edge
                    let mut hops: Vec<(&'g str, f64)> = path.hops.clone();
                    if e.left.as_str() == end && e.right.as_str() != end {
                        hops.push((e.right.as_str(), path.cost() + e.cost));
                    } else if e.left.as_str() != end && e.right.as_str() == end {
                        hops.push((e.left.as_str(), path.cost() + e.cost));
                    } else {
                        continue;
                    }

                    // Add it but ordered by cost
                    todo.insert(Path { hops });
                }
            }
        }

        // OK, done
        shortest
    }
}
