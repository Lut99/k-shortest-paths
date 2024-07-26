//  MAIN.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 01:44:40
//  Last edited:
//    26 Jul 2024, 02:22:42
//  Auto updated?
//    Yes
//
//  Description:
//!   Entrypoint for the `visualize` binary.
//

use std::borrow::Cow;
use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use error_trace::trace;
use humanlog::{DebugMode, HumanLogger};
use image::{ImageFormat, RgbaImage};
use ksp_graph::{Graph, GraphFormat};
use ksp_vis::render::{render_graph, Options};
use log::{debug, error, info};


/***** ARGUMENTS *****/
/// Defines the arguments to the `visualize`-binary.
#[derive(Debug, Parser)]
struct Arguments {
    /// Whether to run with additional log statements.
    #[clap(long, global = true, help = "If given, shows DEBUG- and INFO-level log statements.")]
    debug: bool,
    /// Whether to run with maximum log statements.
    #[clap(long, global = true, help = "If given, shows TRACE-level log statements. Implies '--debug'.")]
    trace: bool,

    /// Any specific files to visualize.
    #[clap(name = "GRAPH", help = "The graph file to visualize.")]
    graph:  PathBuf,
    #[clap(
        short,
        long,
        help = "If given, parses the given file according to the given format. Otherwise, it is automatically deduced from the given file's \
                extension. Recognized extensions are: 'json', 'sndlib'"
    )]
    format: Option<GraphFormat>,
    /// The output file to write the visualization to.
    #[clap(short, long, default_value = "./output.png", help = "The path to write the graph visualization to.")]
    output: PathBuf,
}





/***** ENTRYPOINT *****/
fn main() {
    // Parse the arguments
    let args = Arguments::parse();

    // Setup the logger
    if let Err(err) = HumanLogger::terminal(DebugMode::from_flags(args.trace, args.debug)).init() {
        eprintln!("WARNING: Failed to setup logger: {err} (no logging for this session)");
    }
    info!("{} - v{}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));

    // Resolve the format
    let fmt: GraphFormat = match args.format {
        Some(fmt) => fmt,
        None => {
            debug!("Deducing graph format from '{}'", args.graph.display());
            let sgraph: Cow<str> = args.graph.to_string_lossy();
            if sgraph.ends_with(".json") {
                GraphFormat::Json
            } else if sgraph.ends_with(".xml") {
                GraphFormat::SNDLibXml
            } else {
                error!("Unknown graph format extension{}", if let Some(ext) = args.graph.extension() { format!(" {ext:?}") } else { String::new() });
                std::process::exit(1);
            }
        },
    };

    // Load the graph we're told to load
    debug!("Loading graph file '{}' as {:?}...", args.graph.display(), fmt);
    let g: Graph = match fmt {
        GraphFormat::Json => match ksp_graph::json::parse(&args.graph) {
            Ok(g) => g,
            Err(err) => {
                error!("{}", trace!(("Failed to load graph file '{}' as a JSON graph", args.graph.display()), err));
                std::process::exit(1);
            },
        },
        GraphFormat::SNDLibXml => match ksp_graph::sndlib_xml::parse(&args.graph) {
            Ok(g) => g,
            Err(err) => {
                error!("{}", trace!(("Failed to load graph file '{}' as an SNDLib XML graph", args.graph.display()), err));
                std::process::exit(1);
            },
        },
    };

    // Render
    debug!("Rendering graph...");
    let mut img: RgbaImage = render_graph(&g, Options::default());
    image::imageops::flip_vertical_in_place(&mut img);

    // Write the image
    debug!("Writing rendered image to '{}'...", args.output.display());
    match File::create(&args.output) {
        Ok(mut handle) => {
            if let Err(err) = img.write_to(&mut handle, ImageFormat::Png) {
                error!("{}", trace!(("Failed to write to output image '{}'", args.output.display()), err));
                std::process::exit(1);
            }
        },
        Err(err) => {
            error!("{}", trace!(("Failed to create output image '{}'", args.output.display()), err));
            std::process::exit(1);
        },
    }

    // Done!
    println!("Done.");
}
