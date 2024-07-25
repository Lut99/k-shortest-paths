//  SERDE.rs
//    by Lut99
//
//  Created:
//    19 Jul 2024, 23:35:55
//  Last edited:
//    25 Jul 2024, 22:20:32
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines a parser for [`Graph`]s from a `json`-serialization for
//!   them.
//

use std::collections::HashMap;
use std::error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::fs::File;
use std::path::{Path, PathBuf};

use arrayvec::ArrayString;
use serde::{Deserialize, Serialize};

use crate::{Edge, Graph, Node};


/***** CONSTANTS *****/
/// The default value for bidirectional.
#[inline]
const fn default_bidirectional() -> bool { true }





/***** ERRORS *****/
/// Defines errors originating when [`parse()`]ing JSON-serializations of [`Graph`]s.
#[derive(Debug)]
pub enum Error {
    /// Failed to open the JSON file.
    FileOpen { path: PathBuf, err: std::io::Error },
    /// Failed to read/parse the JSON file.
    FileReadParse { path: PathBuf, err: serde_json::Error },
}
impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FResult {
        use Error::*;
        match self {
            FileOpen { path, .. } => write!(f, "Failed to open graph file '{}'", path.display()),
            FileReadParse { path, .. } => write!(f, "Failed to read & parse graph file '{}' as valid Graph JSON", path.display()),
        }
    }
}
impl error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            FileOpen { err, .. } => Some(err),
            FileReadParse { err, .. } => Some(err),
        }
    }
}





/***** SPEC *****/
/// Defines a graph of nodes linked by edges.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonGraph {
    /// Whether or not the nodes in this graph are bidirectional.
    #[serde(default = "default_bidirectional")]
    pub bidirectional: bool,
    /// The nodes in the graph.
    pub nodes: HashMap<ArrayString<64>, JsonNode>,
    /// The edges in the graph.
    pub edges: HashMap<ArrayString<64>, JsonEdge>,
}



/// Defines a node in each graph.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct JsonNode {
    /// If there's any coordinate information available, this will place it in a 2D-space.
    pub pos: (f64, f64),
}

/// Defines a link between nodes in each graph.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct JsonEdge {
    /// The ID of the first [`Node`] this edge connects.
    pub left:  ArrayString<64>,
    /// The ID of the second [`Node`] this edge connects.
    pub right: ArrayString<64>,
    /// The cost associated with traversing the edge.
    pub cost:  f64,
}





/***** LIBRARY *****/
/// Parses a graph from a JSON file.
///
/// # Arguments
/// - `path`: The [`Path`]-like of the file to parse from.
///
/// # Returns
/// A new [`Graph`] parsed from the given file.
///
/// # Errors
/// This function errors if we failed to open, read or parse the given file.
#[inline]
pub fn parse(path: impl AsRef<Path>) -> Result<Graph, Error> {
    let path: &Path = path.as_ref();
    let mut graph: JsonGraph = match File::open(path) {
        Ok(handle) => match serde_json::from_reader(handle) {
            Ok(graph) => graph,
            Err(err) => return Err(Error::FileReadParse { path: path.into(), err }),
        },
        Err(err) => return Err(Error::FileOpen { path: path.into(), err }),
    };

    // Generate bidirectional links if told to do so
    if graph.bidirectional {
        // Collect the reverse edges
        let mut new_edges: HashMap<ArrayString<64>, JsonEdge> = HashMap::with_capacity(graph.edges.len());
        'edges: for (id, edge) in &graph.edges {
            // Check no such edge exists
            for edge_prime in graph.edges.values() {
                if edge_prime.left == edge.right && edge_prime.right == edge.left {
                    continue 'edges;
                }
            }

            // Add the reverse
            let id: ArrayString<64> = ArrayString::from(&format!("{id}-REV")).unwrap_or_else(|err| panic!("Too long identifier '{id}-REV': {err}"));
            new_edges.insert(id, JsonEdge { left: edge.right.clone(), right: edge.left.clone(), cost: edge.cost });
        }

        // Insert them
        graph.edges.extend(new_edges);
    }

    // OK, translate and return
    Ok(Graph {
        nodes: graph.nodes.into_iter().map(|(id, node)| (id.clone(), Node { id, pos: node.pos })).collect(),
        edges: graph
            .edges
            .into_iter()
            .map(|(id, edge)| (id.clone(), Edge { id, left: edge.left.clone(), right: edge.right.clone(), cost: edge.cost }))
            .collect(),
    })
}
