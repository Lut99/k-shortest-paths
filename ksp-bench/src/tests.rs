//  TEST.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 02:09:04
//  Last edited:
//    16 Jul 2024, 02:41:41
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines some wrappers for defining test cases.
//

use arrayvec::ArrayString;


/***** LIBRARY *****/
/// Defines a testcase.
#[derive(Clone, Copy, Debug)]
pub struct TestCase {
    /// Some name for the case.
    pub id:     ArrayString<64>,
    /// The ID of the first node to find a path from.
    pub source: ArrayString<64>,
    /// The ID of the second node to find a path to.
    pub target: ArrayString<64>,
    /// The number of paths to find.
    pub k:      usize,
}
