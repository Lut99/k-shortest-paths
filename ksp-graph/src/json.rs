//  SERDE.rs
//    by Lut99
//
//  Created:
//    19 Jul 2024, 23:35:55
//  Last edited:
//    19 Jul 2024, 23:41:46
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines a parser for [`Graph`]s from a `json`-serialization for
//!   them.
//

use std::error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::Graph;


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
    match File::open(path) {
        Ok(handle) => match serde_json::from_reader(handle) {
            Ok(graph) => Ok(graph),
            Err(err) => Err(Error::FileReadParse { path: path.into(), err }),
        },
        Err(err) => Err(Error::FileOpen { path: path.into(), err }),
    }
}
