//  PARSER.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 00:54:32
//  Last edited:
//    16 Jul 2024, 02:41:45
//  Auto updated?
//    Yes
//
//  Description:
//!   Provides a parser for parsing [`Graph`]s from XML benchmark files.
//

use std::error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use arrayvec::ArrayString;
use ksp::graph::{Edge, Graph, Node};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::tests::TestCase;


/***** ERRORS *****/
/// Defines errors originating when parsing SNDLib XML graphs.
#[derive(Debug)]
pub enum Error {
    /// Failed to open the graph file.
    FileOpen { path: PathBuf, err: std::io::Error },
    /// Failed to parse the graph file as XML.
    FileParse { path: PathBuf, err: quick_xml::de::DeError },
}
impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            FileOpen { path, .. } => write!(f, "Failed to open benchmark file '{}'", path.display()),
            FileParse { path, .. } => write!(f, "Failed to parse benchmark file '{}' as SNDLib XML", path.display()),
        }
    }
}
impl error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            FileOpen { err, .. } => Some(err),
            FileParse { err, .. } => Some(err),
        }
    }
}





/***** AUXILLARY *****/
/// Representation of a toplevel [`Graph`] in the XML files.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XmlNetwork {
    /// Defines metadata about each network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<XmlMetadata>,
    /// Defines the network topology.
    #[serde(rename = "networkStructure")]
    pub network_structure: XmlNetworkStructure,
    /// Defines any to-be-searched-for links.
    pub demands: XmlDemands,
}



/// Defines expected metadata flags.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XmlMetadata {
    /// The granularity of time measurements of the graph.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    /// The origin of the graph.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// The year of recording the graph.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// A unit in the graph(?)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}



/// Defines the graph topology in XML files.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XmlNetworkStructure {
    /// The nodes in this graph.
    pub nodes: XmlNodes,
    /// The links in this graph.
    pub links: XmlLinks,
}


/// Defines the list of nodes.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XmlNodes {
    /// The type of the coordinates.
    #[serde(rename = "@coordinatesType")]
    pub coordinates_type: XmlCoordsType,
    /// The nodes in this graph.
    #[serde(rename = "$value")]
    pub nodes: Vec<XmlNode>,
}

/// Representation of the possible coordinations type.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum XmlCoordsType {
    /// It's longitude/latitude.
    #[serde(rename = "geographical")]
    Geographical,
    /// It's in pixels.
    #[serde(rename = "pixel")]
    Pixel,
}

/// Representation of a [`Node`] in the XML files.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct XmlNode {
    /// The identifier of the node.
    #[serde(rename = "@id")]
    pub id: ArrayString<64>,
    /// The (geographical) location of the node.
    pub coordinates: XmlNodeCoords,
}

/// Representation of an [`XmlNode`]'s coordinates in the XML files.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct XmlNodeCoords {
    /// The X-coordinate.
    pub x: f64,
    /// The Y-coordinate.
    pub y: f64,
}


/// Defines the list of links.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XmlLinks {
    /// The links in this graph.
    #[serde(rename = "$value")]
    pub links: Vec<XmlLink>,
}

/// Representation of an [`Edge`] in the XML files.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct XmlLink {
    /// The identifier of the link.
    #[serde(rename = "@id")]
    pub id: ArrayString<64>,
    /// The source node.
    pub source: ArrayString<64>,
    /// The target node.
    pub target: ArrayString<64>,
    /// If present, represents the cost it takes traffic to traverse this edge.
    #[serde(rename = "routingCost")]
    pub routing_cost: Option<f64>,
}



/// Representation of a testcase in the XML files.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XmlDemands {
    /// Defines any to-be-searched-for links.
    #[serde(rename = "$value")]
    pub demands: Vec<XmlDemand>,
}

/// Representation of a testcase in the XML files.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct XmlDemand {
    /// The identifier of the demand.
    #[serde(rename = "@id")]
    pub id: ArrayString<64>,
    /// The source node.
    pub source: ArrayString<64>,
    /// The target node.
    pub target: ArrayString<64>,
    /// The target path cost.
    #[serde(rename = "demandValue")]
    pub demand_value: f64,
}





/***** LIBRARY FUNCTIONS *****/
/// Parses a new [`Graph`] from the given SNDLib XML graph file.
///
/// # Arguments
/// - `path`: The path where the XML file is located.
///
/// # Returns
/// A new [`Graph`], encoding the parsed graph, and a list of [`PathTarget`]s, encoding the desired "test cases".
///
/// # Errors
/// This function may errors if we failed to read the target file or failed to parse it as (the right kind of) XML.
pub fn parse_sndlib_xml(path: impl AsRef<Path>) -> Result<(Graph, Vec<TestCase>), Error> {
    let path: &Path = path.as_ref();

    debug!("Loading benchmark file '{}'...", path.display());
    let bench: XmlNetwork = match File::open(&path) {
        Ok(handle) => match quick_xml::de::from_reader(BufReader::new(handle)) {
            Ok(bench) => bench,
            Err(err) => return Err(Error::FileParse { path: path.into(), err }),
        },
        Err(err) => return Err(Error::FileOpen { path: path.into(), err }),
    };

    // Convert it to the standardized Graph.
    Ok((
        Graph {
            nodes: bench
                .network_structure
                .nodes
                .nodes
                .into_iter()
                .map(|n| (n.id, Node { id: n.id, pos: Some((n.coordinates.x, n.coordinates.y)) }))
                .collect(),
            edges: bench
                .network_structure
                .links
                .links
                .into_iter()
                .map(|l| {
                    // Collect the cost of the node
                    (l.id, Edge { id: l.id, left: l.source, right: l.target, cost: l.routing_cost.unwrap_or(0.0) })
                })
                .collect(),
        },
        bench.demands.demands.into_iter().map(|d| TestCase { id: d.id, source: d.source, target: d.target, k: 1 }).collect(),
    ))
}
