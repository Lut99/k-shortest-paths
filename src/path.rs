//  PATH.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 02:05:23
//  Last edited:
//    16 Jul 2024, 02:23:13
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines a path between two nodes.
//


/***** LIBRARY *****/
/// Defines a path between two nodes.
#[derive(Clone, Debug)]
pub struct Path<'g> {
    /// The hops of the path.
    pub hops: Vec<&'g str>,
    /// The total cost of the path.
    pub cost: f64,
}
impl<'g> Path<'g> {
    /// Returns the endpoint of this path, if any.
    ///
    /// # Returns
    /// A reference to the ID of the endpoint of the path.
    #[inline]
    pub fn end(&self) -> Option<&'g str> { self.hops.last().copied() }
}
