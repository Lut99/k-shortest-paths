//  PIPELINE.rs
//    by Lut99
//
//  Created:
//    25 Jul 2024, 01:05:15
//  Last edited:
//    26 Jul 2024, 01:47:57
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the main [`Pipeline`] data structure.
//

use std::collections::HashMap;
use std::error;
use std::fmt::{Display, Formatter, Result as FResult};

use arrayvec::ArrayString;
use image::RgbaImage;
use ksp_alg::dist::dijkstra::Dijkstra as DijkstraDist;
use ksp_alg::dist::{Distance, Distancing};
use ksp_alg::ksp::Ksp;
use ksp_alg::sssp::dijkstra::Dijkstra as DijkstraSssp;
use ksp_alg::sssp::Sssp;
use ksp_alg::trans::peek::PeeK;
use ksp_alg::trans::{Transformer, Transforming as _};
use ksp_alg::wikipedia::Wikipedia;
use ksp_alg::yen::Yen;
use ksp_alg::{MultiRouting, OwnedPath};
use ksp_graph::Graph;
use ksp_vis::render::Options;
use serde::{Deserialize, Serialize};
use show_image::{create_window, event, WindowProxy};


/***** ERRORS *****/
/// Defines the errors occurring when running [`Pipeline`]s.
#[derive(Debug)]
pub enum Error {
    /// Failed to create window for showing the current graph.
    WindowCreate { err: show_image::error::CreateWindowError },
    /// Failed to set the image in the window for showing the current graph.
    WindowSetImage { dims: (u32, u32), err: show_image::error::SetImageError },
    /// Failed to wait for the image window to be closed.
    WindowWait { err: show_image::error::InvalidWindowId },
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            WindowCreate { .. } => write!(f, "Failed to create graph visualization window"),
            WindowSetImage { dims, .. } => write!(f, "Failed to set image of {}x{} in graph visualization window", dims.0, dims.1),
            WindowWait { .. } => write!(f, "Failed to wait for graph visualization window to close"),
        }
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            WindowCreate { err } => Some(err),
            WindowSetImage { err, .. } => Some(err),
            WindowWait { err } => Some(err),
        }
    }
}





/***** LIBRARY *****/
/// Defines a whole Pipeline.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Pipeline {
    /// The name of the pipeline.
    pub name: String,
    /// Defines the steps to take.
    steps:    Vec<PipelineStep>,
}
impl Pipeline {
    /// Constructor for a Pipeline that initializes it with a given input graph.
    ///
    /// This constructor will automatically deduce the graph format from its extension.
    ///
    /// # Arguments
    /// - `name`: Some name used for debugging pipelines.
    ///
    /// # Returns
    /// A new Pipeline instance.
    #[inline]
    pub fn new(name: impl Into<String>) -> Self { Self { name: name.into(), steps: vec![] } }



    /// Adds a new step to the pipeline.
    ///
    /// # Arguments
    /// - `step`: Some [`PipelineStep`] to add.
    ///
    /// # Returns
    /// A mutable reference to self for chaining.
    #[inline]
    pub fn add_step(&mut self, step: impl Into<PipelineStep>) -> &mut Self {
        self.steps.push(step.into());
        self
    }



    /// Executes a full run of the pipeline.
    ///
    /// # Arguments
    /// - `graph`: The [`Graph`] to find in.
    /// - `src`: The source node to find a path from.
    /// - `dst`: The destination node to find a path to.
    /// - `k`: The number of paths to find.
    ///
    /// # Returns
    /// A list of the `k` shortest paths found, or [`None`] if no K-Shortest Path algorithm was executed.
    ///
    /// # Panics
    /// This function is allowed to panic if the given `src` or `dst` are not in the given `graph` or they are not connected.
    pub fn k_shortest(&self, graph: &Graph, src: &str, dst: &str, k: usize) -> Result<Option<Vec<OwnedPath>>, Error> {
        // Run through the steps
        let mut graph: Graph = graph.clone();
        let mut res: Option<Vec<OwnedPath>> = None;
        for step in &self.steps {
            match step {
                // Transformers
                PipelineStep::Transform(PipelineStepTransform { trans: Transformer::PeeK(Distance::Dijkstra) }) => {
                    PeeK::<DijkstraDist>::transform(&mut graph, src, dst, k)
                },

                // Visualize
                PipelineStep::Visualize(PipelineStepVisualize { labels: NodeLabels::Identifiers }) => {
                    // Render the graph and continue
                    let mut img: RgbaImage = ksp_vis::render::render_graph(&graph, Options::default());
                    image::imageops::flip_vertical_in_place(&mut img);
                    let window: WindowProxy = match create_window("Graph", Default::default()) {
                        Ok(window) => window,
                        Err(err) => return Err(Error::WindowCreate { err }),
                    };
                    let dims: (u32, u32) = (img.width(), img.height());
                    if let Err(err) = window.set_image("Graph", img) {
                        return Err(Error::WindowSetImage { dims, err });
                    }
                    for event in window.event_channel().map_err(|err| Error::WindowWait { err })? {
                        if let event::WindowEvent::KeyboardInput(event) = event {
                            if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                                break;
                            }
                        } else if let event::WindowEvent::CloseRequested(_) = event {
                            break;
                        }
                    }
                },
                PipelineStep::Visualize(PipelineStepVisualize {
                    labels: NodeLabels::Distance(NodeLabelsDistance { dist: Distance::Dijkstra, node }),
                }) => {
                    // Compute the graph colouring first
                    let dist: HashMap<&str, f64> = DijkstraDist::shortest_all(&graph, node);

                    // Render the graph and continue
                    let mut img: RgbaImage = ksp_vis::render::render_graph_with_labels(
                        &graph,
                        &dist.into_iter().map(|(id, dist)| (id, dist.to_string())).collect(),
                        Options::default(),
                    );
                    image::imageops::flip_vertical_in_place(&mut img);
                    let window: WindowProxy = match create_window("Graph", Default::default()) {
                        Ok(window) => window,
                        Err(err) => return Err(Error::WindowCreate { err }),
                    };
                    let dims: (u32, u32) = (img.width(), img.height());
                    if let Err(err) = window.set_image("Graph", img) {
                        return Err(Error::WindowSetImage { dims, err });
                    }
                    for event in window.event_channel().map_err(|err| Error::WindowWait { err })? {
                        if let event::WindowEvent::KeyboardInput(event) = event {
                            if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                                break;
                            }
                        } else if let event::WindowEvent::CloseRequested(_) = event {
                            break;
                        }
                    }
                    // if let Err(err) = window.wait_until_destroyed() {
                    //     return Err(Error::WindowWait { err });
                    // }
                    println!("HOWDY");
                },

                // K-Shortest paths
                PipelineStep::KShortestPath(PipelineStepKSP { ksp: Ksp::Wikipedia }) => {
                    res = Some(Wikipedia::k_shortest(&graph, src, dst, k).into_iter().map(|p| p.to_owned()).collect())
                },
                PipelineStep::KShortestPath(PipelineStepKSP { ksp: Ksp::Yen(Sssp::Dijkstra) }) => {
                    res = Some(Yen::<DijkstraSssp>::k_shortest(&graph, src, dst, k).into_iter().map(|p| p.to_owned()).collect())
                },
            }
        }
        Ok(res)
    }
}

/// Defines a possible step in a [`Pipeline`].
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PipelineStep {
    /// Apply a KSP algorithm.
    KShortestPath(PipelineStepKSP),
    /// Apply a graph transformation.
    Transform(PipelineStepTransform),
    /// Visualize the current graph.
    Visualize(PipelineStepVisualize),
}
impl From<PipelineStepKSP> for PipelineStep {
    #[inline]
    fn from(value: PipelineStepKSP) -> Self { Self::KShortestPath(value) }
}
impl From<PipelineStepTransform> for PipelineStep {
    #[inline]
    fn from(value: PipelineStepTransform) -> Self { Self::Transform(value) }
}
impl From<PipelineStepVisualize> for PipelineStep {
    #[inline]
    fn from(value: PipelineStepVisualize) -> Self { Self::Visualize(value) }
}



/// Defines a step in a [`Pipeline`] that computes the K shortest paths between two nodes.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PipelineStepKSP {
    /// The algorithm to execute.
    pub ksp: Ksp,
}



/// Defines a step in a [`Pipeline`] that transforms the graph.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PipelineStepTransform {
    /// The the transformer to apply.
    pub trans: Transformer,
}



/// Defines a step in a [`Pipeline`] that visualizes the current graph.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PipelineStepVisualize {
    /// What to visualize for nodes
    pub labels: NodeLabels,
}

/// Defines what to label nodes with.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum NodeLabels {
    /// Annotate them with their identifiers.
    Identifiers,
    /// Annotate them with distance colors to the given node, computed with the given algorithm.
    Distance(NodeLabelsDistance),
}

/// Defines the settings for annotating with distance labels.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct NodeLabelsDistance {
    /// The algorithm to use to compute the distance.
    pub dist: Distance,
    /// The node to compute the distance to.
    pub node: ArrayString<64>,
}
