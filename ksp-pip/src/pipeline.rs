//  PIPELINE.rs
//    by Lut99
//
//  Created:
//    25 Jul 2024, 01:05:15
//  Last edited:
//    26 Jul 2024, 02:22:15
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the main [`Pipeline`] data structure.
//

use std::collections::HashMap;
use std::error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Output};

use arrayvec::ArrayString;
use image::{ImageFormat, RgbaImage};
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


/***** ERRORS *****/
/// Defines the errors occurring when running [`Pipeline`]s.
#[derive(Debug)]
pub enum Error {
    /// Failed to save an image to disk.
    ImageSave { path: PathBuf, fmt: ImageFormat, err: image::error::ImageError },
    /// The subprocess was launched but failed on its own accord.
    SubprocessFailed { cmd: Command, status: ExitStatus, stdout: Vec<u8>, stderr: Vec<u8> },
    /// Failed to launch a subprocess.
    SubprocessSpawn { cmd: Command, err: std::io::Error },
    /// We're running on an unknown OS
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    UnknownOs,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            ImageSave { path, fmt, .. } => write!(f, "Failed to save image to '{}' as {:?}", path.display(), fmt),
            SubprocessFailed { cmd, status, stdout, stderr } => write!(
                f,
                "Subprocess {:?} failed with exit status {:?}\n\nstdout:\n{}\n{}\n{}\n\nstderr:\n{}\n{}\n{}\n",
                cmd,
                status,
                (0..80).map(|_| '-').collect::<String>(),
                String::from_utf8_lossy(stdout),
                (0..80).map(|_| '-').collect::<String>(),
                (0..80).map(|_| '-').collect::<String>(),
                String::from_utf8_lossy(stderr),
                (0..80).map(|_| '-').collect::<String>()
            ),
            SubprocessSpawn { cmd, .. } => write!(f, "Failed to span process with {cmd:?}"),
            #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
            UnknownOs => write!(f, "You are running on an unknown OS. This application does not know how to show you images in that case."),
        }
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            ImageSave { err, .. } => Some(err),
            SubprocessFailed { .. } => None,
            SubprocessSpawn { err, .. } => Some(err),
            #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
            UnknownOs => None,
        }
    }
}





/***** HELPER FUNCTIONS *****/
/// Shows the user an image and blocks until the user stops doing so.
///
/// # Arguments
/// - `image`: The image to show.
///
/// # Errors
/// This function may error if we failed to save the image to a temporary location on-disk or if
/// the OS' file opener failed.
fn show_image(mut img: RgbaImage) -> Result<(), Error> {
    // Flip da image
    image::imageops::flip_vertical_in_place(&mut img);

    // Write the image to the tempdir
    let img_path: PathBuf = std::env::temp_dir().join("graph.png");
    if let Err(err) = img.save_with_format(&img_path, ImageFormat::Png) {
        return Err(Error::ImageSave { path: img_path, fmt: ImageFormat::Png, err });
    }

    // Open in different ways
    #[cfg(target_os = "windows")]
    let mut cmd: Command = Command::new("open");
    #[cfg(target_os = "macos")]
    let mut cmd: Command = Command::new("open");
    #[cfg(target_os = "linux")]
    let mut cmd: Command = Command::new("xdg-open");
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return Err(Error::UnknownOs);

    // Append the file to open
    cmd.arg(&img_path);

    // Execute it
    let out: Output = match cmd.output() {
        Ok(out) => out,
        Err(err) => return Err(Error::SubprocessSpawn { cmd, err }),
    };
    if !out.status.success() {
        return Err(Error::SubprocessFailed { cmd, status: out.status, stdout: out.stdout, stderr: out.stderr });
    }

    // OK, done
    Ok(())
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
                    // Render the graph, show it, and continue when their image is closed
                    show_image(ksp_vis::render::render_graph(&graph, Options::default()))?;
                },
                PipelineStep::Visualize(PipelineStepVisualize {
                    labels: NodeLabels::Distance(NodeLabelsDistance { dist: Distance::Dijkstra, node }),
                }) => {
                    // Compute the graph colouring first
                    let dist: HashMap<&str, f64> = DijkstraDist::shortest_all(&graph, node);

                    // Render the graph, show it, and continue when their image is closed
                    show_image(ksp_vis::render::render_graph_with_labels(
                        &graph,
                        &dist.into_iter().map(|(id, dist)| (id, dist.to_string())).collect(),
                        Options::default(),
                    ))?;
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
