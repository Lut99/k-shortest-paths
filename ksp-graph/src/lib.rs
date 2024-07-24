//  LIB.rs
//    by Lut99
//
//  Created:
//    19 Jul 2024, 23:35:02
//  Last edited:
//    25 Jul 2024, 00:34:30
//  Auto updated?
//    Yes
//
//  Description:
//!   <Todo>
//

// Declare sub-modules
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "sndlib_xml")]
pub mod sndlib_xml;

// Imports
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::str::FromStr;

use arrayvec::ArrayString;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};


/***** ERRORS *****/
/// Defines errors from parsing [`GraphFormat`]s from strings.
#[derive(Debug)]
pub struct GraphFormatParseError {
    /// The string we didn't recognize.
    unknown: String,
}
impl Display for GraphFormatParseError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FResult { write!(f, "Unknown graph format identifier '{}'", self.unknown) }
}
impl Error for GraphFormatParseError {}





/***** AUXILLARY *****/
/// Lists all available graph formats we can parse.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GraphFormat {
    /// A directly serialized [`Graph`] as JSON.
    #[cfg(feature = "json")]
    Json,
    /// An XML description of SNDLib networks.
    #[cfg(feature = "sndlib_xml")]
    SNDLibXml,
}
impl GraphFormat {
    /// Returns a list of all supported formats.
    ///
    /// # Returns
    /// A `'static` slice of all formats.
    #[inline]
    pub const fn all() -> &'static [Self] {
        &[
            #[cfg(feature = "json")]
            Self::Json,
            #[cfg(feature = "sndlib_xml")]
            Self::SNDLibXml,
        ]
    }
}
impl FromStr for GraphFormat {
    type Err = GraphFormatParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            #[cfg(feature = "json")]
            "json" => Ok(Self::Json),
            #[cfg(feature = "sndlib_xml")]
            "sndlib_xml" => Ok(Self::SNDLibXml),
            unknown => Err(GraphFormatParseError { unknown: unknown.into() }),
        }
    }
}





/***** LIBRARY *****/
/// Defines a graph of nodes linked by edges.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Deserialize, Serialize))]
pub struct Graph {
    /// The nodes in the graph.
    pub nodes: HashMap<ArrayString<64>, Node>,
    /// The edges in the graph.
    pub edges: HashMap<ArrayString<64>, Edge>,
}



/// Defines a node in each graph.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "json", derive(Deserialize, Serialize))]
pub struct Node {
    /// The identifier of the node.
    pub id:  ArrayString<64>,
    /// If there's any coordinate information available, this will place it in a 2D-space.
    pub pos: (f64, f64),
}

/// Defines a link between nodes in each graph.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "json", derive(Deserialize, Serialize))]
pub struct Edge {
    /// The identifier of the edge.
    pub id:    ArrayString<64>,
    /// The ID of the first [`Node`] this edge connects.
    pub left:  ArrayString<64>,
    /// The ID of the second [`Node`] this edge connects.
    pub right: ArrayString<64>,
    /// The cost associated with traversing the edge.
    pub cost:  f64,
}
