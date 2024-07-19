//  PARSER.rs
//    by Lut99
//
//  Created:
//    19 Jul 2024, 23:47:38
//  Last edited:
//    19 Jul 2024, 23:53:05
//  Auto updated?
//    Yes
//
//  Description:
//!   Parses test cases from an SNDLib XML file.
//

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub use ksp_graph::sndlib_xml::Error;
use ksp_graph::sndlib_xml::XmlNetwork;

use crate::tests::TestCase;


/***** LIBRARY FUNCTIONS *****/
/// Parses any demands in the SNDLib XML file as [`TestCase`]s.
///
/// # Arguments
/// - `path`: The path where the XML file is located.
///
/// # Returns
/// A list of [`TestCase`]s, encoding the desired "test cases".
///
/// # Errors
/// This function may error if we failed to read the target file or failed to parse it as (the right kind of) XML.
pub fn parse_tests(path: impl AsRef<Path>) -> Result<Vec<TestCase>, Error> {
    let path: &Path = path.as_ref();

    // Open & parse the file
    let bench: XmlNetwork = match File::open(&path) {
        Ok(handle) => match quick_xml::de::from_reader(BufReader::new(handle)) {
            Ok(bench) => bench,
            Err(err) => return Err(Error::FileReadParse { path: path.into(), err }),
        },
        Err(err) => return Err(Error::FileOpen { path: path.into(), err }),
    };

    // Convert it to the standardized Graph.
    Ok(bench.demands.demands.into_iter().map(|d| TestCase { id: d.id, source: d.source, target: d.target, k: 1 }).collect())
}
