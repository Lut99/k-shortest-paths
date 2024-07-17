//  PATH.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 02:05:23
//  Last edited:
//    18 Jul 2024, 00:00:22
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines a path between two nodes.
//

use std::hash::{Hash, Hasher};


/***** LIBRARY *****/
/// Defines a path between two nodes.
#[derive(Clone, Debug)]
pub struct Path<'g> {
    /// The hops of the path.
    pub hops: Vec<(&'g str, f64)>,
}
impl<'g> Path<'g> {
    /// Returns the endpoint of this path, if any.
    ///
    /// # Returns
    /// A reference to the ID of the endpoint of the path.
    #[inline]
    pub fn end(&self) -> Option<&'g str> { self.hops.last().map(|(n, _)| *n) }

    /// Returns the cost of this path.
    ///
    /// # Returns
    /// The cost of the entire path.
    #[inline]
    pub fn cost(&self) -> f64 { self.hops.last().map(|(_, c)| *c).unwrap_or(0.0) }
}

impl<'g> Eq for Path<'g> {}
impl<'g> Hash for Path<'g> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (hop, _) in &self.hops {
            hop.hash(state)
        }
    }
}
impl<'g> PartialEq for Path<'g> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.hops.len() != other.hops.len() {
            return false;
        }
        for i in 0..self.hops.len() {
            if self.hops[i].0 != other.hops[i].0 {
                return false;
            }
        }
        return true;
    }
}
