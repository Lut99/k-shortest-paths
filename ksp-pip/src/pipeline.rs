//  PIPELINE.rs
//    by Lut99
//
//  Created:
//    25 Jul 2024, 01:05:15
//  Last edited:
//    25 Jul 2024, 01:09:50
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the main [`Pipeline`] data structure.
//

use ksp_alg::Algorithm;
use serde::{Deserialize, Serialize};


/***** LIBRARY *****/
/// Defines a whole Pipeline.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Pipeline {
    /// Defines the steps to take.
    pub steps: Vec<PipelineStep>,
}

/// Defines a possible step in a [`Pipeline`].
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum PipelineStep {
    /// Apply a KSP algorithm.
    KShortestPath(PipelineStepKSP),
    /// Apply a graph transformation.
    Transform(PipelineStepTransform),
    /// Visualize the current graph.
    Visualize(PipelineStepVisualize),
}



/// Defines a step in a [`Pipeline`] that computes the K shortest paths between two nodes.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PipelineStepKSP {
    /// The algorithm to execute.
    pub alg: Algorithm,
}



/// Defines a step in a [`Pipeline`] that transforms the graph.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PipelineStepTransform {
    /// The the transformer to apply.
    pub trans: Transformer,
}
