//  VANILLA.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 00:10:52
//  Last edited:
//    25 Jul 2024, 01:13:51
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the simplest KSP algorithm as presented by Wikipedia.
//!   
//!   Based on: <https://en.wikipedia.org/wiki/K_shortest_path_routing#Algorithm>
//

use std::collections::HashMap;

use arrayvec::ArrayString;
use ksp_graph::Graph;

use super::MultiRouting;
use crate::path::Path;


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
impl MultiRouting for WikipediaKSP {
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
        // > P = empty,
        let mut shortest: Vec<Path<'g>> = Vec::with_capacity(k);
        // > count_u = 0, for all u in V
        let mut shortest_to: HashMap<&str, usize> = HashMap::with_capacity(graph.nodes.len());
        // > insert path p_s = {s} into B with cost 0
        let mut todo: Vec<Path<'g>> = Vec::from([Path { hops: vec![(src, 0.0)] }]);
        // > while B is not empty and count_t < K:
        while !todo.is_empty() && *shortest_to.entry(dst).or_default() < k {
            // > let p_u be the shortest cost path in B with cost C
            // > B = B - {p_u},
            let path: Path<'g> = todo.pop().unwrap();
            let cost: f64 = path.cost();
            let end: &str = path.end().unwrap();

            // > count_u = count_u + 1
            *shortest_to.entry(end).or_default() += 1;

            // > if u = t then P = P \cup {p_u}
            if dst == end {
                shortest.push(path.clone());
            }

            // > if count_u \leq K then
            if *shortest_to.get(end).unwrap() <= k {
                // > \circ for each vertex v adjacent to u:
                'edges: for e in graph.edges.values() {
                    // > - let p_v be a new path with cost C + w(u, v) formed by concatenating edge (u, v) to path p_u
                    let neighbour: &str = if e.left.as_str() == end && e.right.as_str() != end {
                        e.right.as_str()
                    } else if e.left.as_str() != end && e.right.as_str() == end {
                        e.left.as_str()
                    } else {
                        continue;
                    };
                    let new_cost: f64 = cost + e.cost;
                    let mut new_path: Path<'g> = path.clone();
                    new_path.hops.push((neighbour, cost + e.cost));

                    // > - insert p_v into B
                    // NOTE: We do this ordered
                    for i in 0..todo.len() {
                        // Insert it after the largest one
                        if todo[i].cost() > new_cost {
                            continue;
                        }
                        todo.insert(i, new_path);
                        continue 'edges;
                    }
                    todo.push(new_path);
                }
            }
        }

        // > return P
        shortest
    }
}
