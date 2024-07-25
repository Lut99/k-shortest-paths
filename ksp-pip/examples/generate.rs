//  GENERATE.rs
//    by Lut99
//
//  Created:
//    26 Jul 2024, 01:12:16
//  Last edited:
//    26 Jul 2024, 01:21:24
//  Auto updated?
//    Yes
//
//  Description:
//!   Generates some default pipeline files.
//

use std::fs::{self, File};
use std::path::PathBuf;

use error_trace::trace;
use humanlog::{DebugMode, HumanLogger};
use ksp_alg::dist::Distance;
use ksp_alg::sssp::Sssp;
use ksp_alg::trans::Transformer;
use ksp_alg::Ksp;
use ksp_pip::{NodeLabels, Pipeline, PipelineStepKSP, PipelineStepTransform, PipelineStepVisualize};
use log::{error, info};


/***** ENTRYPOINT *****/
fn main() {
    // Setup the logger
    if let Err(err) = HumanLogger::terminal(DebugMode::HumanFriendly).init() {
        eprintln!("WARNING: Failed to setup logger: {err} (logging disabled for this session)");
    }
    info!("{} - v{}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));

    // Check if the output dir exists
    let pipelines_path: PathBuf = PathBuf::from("./pipelines");
    if !pipelines_path.exists() {
        if let Err(err) = fs::create_dir_all(&pipelines_path) {
            error!("{}", trace!(("Failed to create directory '{}'", pipelines_path.display()), err));
            std::process::exit(1);
        }
    }



    // Pipeline #1
    let pipeline_path: PathBuf = pipelines_path.join("peek-yen-dijkstra.json");
    let mut pipeline: Pipeline = Pipeline::new("PeeK-Yen-Dijkstra");
    pipeline
        .add_step(PipelineStepTransform { trans: Transformer::PeeK(Distance::Dijkstra) })
        .add_step(PipelineStepKSP { ksp: Ksp::Yen(Sssp::Dijkstra) });
    match File::create(&pipeline_path) {
        Ok(handle) => {
            if let Err(err) = serde_json::to_writer_pretty(handle, &pipeline) {
                error!("{}", trace!(("Failed to serialize/write pipeline '{}' to '{}'", pipeline.name, pipeline_path.display()), err));
                std::process::exit(1);
            }
        },
        Err(err) => {
            error!("{}", trace!(("Failed to create pipeline file '{}'", pipeline_path.display()), err));
            std::process::exit(1);
        },
    }



    // Pipeline #2
    let pipeline_path: PathBuf = pipelines_path.join("peek-yen-dijkstra-debug.json");
    let mut pipeline: Pipeline = Pipeline::new("PeeK-Yen-Dijkstra (DEBUG)");
    pipeline
        .add_step(PipelineStepVisualize { labels: NodeLabels::Identifiers })
        .add_step(PipelineStepTransform { trans: Transformer::PeeK(Distance::Dijkstra) })
        .add_step(PipelineStepVisualize { labels: NodeLabels::Identifiers })
        .add_step(PipelineStepKSP { ksp: Ksp::Yen(Sssp::Dijkstra) });
    match File::create(&pipeline_path) {
        Ok(handle) => {
            if let Err(err) = serde_json::to_writer_pretty(handle, &pipeline) {
                error!("{}", trace!(("Failed to serialize/write pipeline '{}' to '{}'", pipeline.name, pipeline_path.display()), err));
                std::process::exit(1);
            }
        },
        Err(err) => {
            error!("{}", trace!(("Failed to create pipeline file '{}'", pipeline_path.display()), err));
            std::process::exit(1);
        },
    }
}
