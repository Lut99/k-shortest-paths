//  UTILS.rs
//    by Lut99
//
//  Created:
//    20 Jul 2024, 01:05:09
//  Last edited:
//    20 Jul 2024, 01:11:14
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines utilities for use in tests.
//

use std::path::PathBuf;

use error_trace::trace;
use ksp_graph::Graph;


/***** LIBRARY *****/
/// Loads a test graph with a given name.
///
/// # Arguments
/// - `name`: The name of the file to load. Doesn't need to include `.json` (but it can).
///
/// # Returns
/// A loaded [`Graph`].
///
/// # Panics
/// This function panics if it failed to load the given file.
pub fn load_graph(name: impl AsRef<str>) -> Graph {
    let name: &str = name.as_ref();

    // Check if the file exists without mods
    let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join(name);
    if !path.exists() {
        path.set_file_name(format!("{name}.json"));
    }

    // OK try to do it
    match ksp_graph::json::parse(&path) {
        Ok(g) => g,
        Err(err) => panic!("{}", trace!(("Failed to load graph file '{}'", path.display()), err)),
    }
}
