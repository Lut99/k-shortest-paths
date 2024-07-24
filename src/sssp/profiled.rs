//  PROFILED.rs
//    by Lut99
//
//  Created:
//    24 Jul 2024, 20:41:44
//  Last edited:
//    24 Jul 2024, 20:53:31
//  Auto updated?
//    Yes
//
//  Description:
//!   A phony SSSP implementation that wraps another and reports its
//!   timings everytime its called.
//

use std::time::{Duration, Instant};

use ksp_graph::Graph;

use super::SingleShortestPath;
use crate::path::Path;


/***** LIBRARY *****/
/// A wrapper around other SSSP implementations that will profile its calls.
pub struct ProfilingSSSP<S> {
    /// The nested SSSP itself.
    sssp: S,
    /// Where to record the timings.
    pub timings: Vec<Duration>,
}
impl<S> ProfilingSSSP<S> {
    /// Constructor for the ProfilingSSSP.
    ///
    /// # Arguments
    /// - `sssp`: The SSSP algorithm to wrap.
    ///
    /// # Returns
    /// A new ProfilingSSSP instance.
    #[inline]
    pub const fn new(sssp: S) -> Self { Self { sssp, timings: vec![] } }
}
impl<S: SingleShortestPath> SingleShortestPath for ProfilingSSSP<S> {
    #[track_caller]
    fn shortest<'g>(&mut self, graph: &'g Graph, src: &str, dst: &str) -> Path<'g> {
        // Record the run
        let start: Instant = Instant::now();
        let path: Path<'g> = self.sssp.shortest(graph, src, dst);
        let time: Duration = start.elapsed();

        // Store it internally before completing
        if self.timings.len() >= self.timings.capacity() {
            self.timings.reserve(self.timings.len());
        }
        self.timings.push(time);
        path
    }
}
