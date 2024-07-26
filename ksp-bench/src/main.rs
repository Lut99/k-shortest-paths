//  MAIN.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 00:09:40
//  Last edited:
//    26 Jul 2024, 02:25:19
//  Auto updated?
//    Yes
//
//  Description:
//!   Entrypoint for the `benchmark`-binary.
//

use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::{self, DirEntry, File, ReadDir};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use arrayvec::ArrayString;
use clap::Parser;
use comfy_table::Table;
use error_trace::trace;
use humanlog::{DebugMode, HumanLogger};
use ksp_alg::OwnedPath;
use ksp_bench::parser::{self};
use ksp_bench::tests::TestCase;
use ksp_graph::{Graph, GraphFormat};
use ksp_pip::Pipeline;
use log::{debug, error, info, warn};


/***** ARGUMENTS *****/
/// Defines the binary arguments.
#[derive(Clone, Debug, Parser)]
struct Arguments {
    /// Whether to run with additional log statements.
    #[clap(long, global = true, help = "If given, shows DEBUG- and INFO-level log statements.")]
    debug: bool,
    /// Whether to run with maximum log statements.
    #[clap(long, global = true, help = "If given, shows TRACE-level log statements. Implies '--debug'.")]
    trace: bool,

    /// Any algorithms to run.
    #[clap(name = "PIPELINES", help = "A list of KSP pipelines to benchmark. They are given as JSON files.")]
    pips: Vec<PathBuf>,
    /// Any specific benchmarks to run.
    #[clap(
        short,
        long,
        help = "If given, does not run all benchmarks in the '--benchmark-dir', but instead only the ones with the given name. Adding '.xml' is \
                optional."
    )]
    benchmark: Vec<String>,
    /// Any specific tests to run.
    #[clap(short, long, help = "If given, runs only the given tests in each benchmark file.")]
    test: Vec<ArrayString<64>>,
    /// Where to find the benchmarks.
    #[clap(short = 'd', long, default_value = "./benchmarks", help = "The directory where the benchmark XML files are read from.")]
    benchmark_dir: PathBuf,

    /// If given, prints the results as CSV.
    #[clap(short, long, help = "If given, prints the results as Comma-Separated Values (CSV) instead of in a table.")]
    csv: bool,
}





/***** ENTRYPOINT *****/
fn main() {
    // Parse arguments
    let args = Arguments::parse();

    // Setup the logger
    if let Err(err) = HumanLogger::terminal(DebugMode::from_flags(args.trace, args.debug)).init() {
        eprintln!("WARNING: Failed to setup logger: {err} (no logging for this session)");
    }
    info!("{} -  v{}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));



    // Parse the pipelines
    let mut pipelines: Vec<Pipeline> = Vec::with_capacity(args.pips.len());
    for pip in args.pips {
        // Open the file
        let handle: File = match File::open(&pip) {
            Ok(handle) => handle,
            Err(err) => {
                error!("{}", trace!(("Failed to open pipeline file '{}'", pip.display()), err));
                std::process::exit(1);
            },
        };

        // Parse as JSON
        let pip: Pipeline = match serde_json::from_reader(handle) {
            Ok(pip) => pip,
            Err(err) => {
                error!("{}", trace!(("Failed to read/parse pipeline file '{}'", pip.display()), err));
                std::process::exit(1);
            },
        };

        // Store
        pipelines.push(pip);
    }



    // Resolve to a list of benchmark files
    let mut files: Vec<(String, PathBuf, GraphFormat)> = Vec::new();
    if !args.benchmark.is_empty() {
        // Resolve the entries
        for entry in args.benchmark {
            let entry_path: PathBuf = PathBuf::from(&entry);
            if entry_path.exists() {
                let entry_name: String = entry_path.file_name().map(|n| n.to_string_lossy().into()).unwrap_or(entry);
                let fmt: GraphFormat = if entry_name.ends_with(".json") { GraphFormat::Json } else { GraphFormat::SNDLibXml };
                files.push((entry_name, entry_path, fmt));
            } else {
                let mut path: PathBuf = args.benchmark_dir.join(&entry);
                if !path.exists() {
                    // Re-try with XML
                    if let Some(old) = path.file_name() {
                        let mut old = old.to_os_string();
                        old.push(".xml");
                        path.set_file_name(old);
                    } else {
                        panic!("Should never have no filename for generated path '{}' from entry '{}'", path.display(), entry);
                    }
                    if !path.exists() {
                        error!("Benchmark '{}' ({}) does not exist", entry, path.display());
                        std::process::exit(1);
                    }
                }
                files.push((entry, path, GraphFormat::SNDLibXml));
            }
        }
    } else {
        debug!("Finding benchmarks (none specified)...");

        // Attempt to read the directory
        let entries: ReadDir = match fs::read_dir(&args.benchmark_dir) {
            Ok(entries) => entries,
            Err(err) => {
                error!("{}", trace!(("Failed to read benchmark directory '{}'", args.benchmark_dir.display()), err));
                std::process::exit(1);
            },
        };

        // Collect the entries
        for (i, entry) in entries.enumerate() {
            // Unwrap it
            let entry: DirEntry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    error!("{}", trace!(("Failed to read benchmark directory '{}' entry {}", args.benchmark_dir.display(), i), err));
                    std::process::exit(1);
                },
            };

            // See if it ends in .xml
            let entry_path: PathBuf = entry.path();
            let entry_name: Cow<str> = entry_path.file_name().unwrap_or_else(|| entry_path.as_os_str()).to_string_lossy();
            if !entry_name.ends_with(".xml") {
                debug!("Skipping entry '{}' as it does not end in '.xml'", entry_path.display());
                continue;
            }

            // Add it
            files.push((entry_path.file_stem().map(|n| n.to_string_lossy().into()).unwrap_or(i.to_string()), entry_path, GraphFormat::SNDLibXml));
        }
    }



    // Run them
    debug!("Running {} benchmark(s)", files.len());
    let mut first: bool = true;
    for (name, file, fmt) in files {
        debug!("Loading benchmark {:?} @ '{}' as {:?}...", name, file.display(), fmt);

        // Open the file and parse the graph & test case
        let mut graph: Graph = match fmt {
            GraphFormat::SNDLibXml => match ksp_graph::sndlib_xml::parse(&file) {
                Ok(res) => res,
                Err(err) => {
                    error!("{}", trace!(("Failed to load benchmark '{name}'"), err));
                    std::process::exit(1);
                },
            },
            GraphFormat::Json => match ksp_graph::json::parse(&file) {
                Ok(res) => res,
                Err(err) => {
                    error!("{}", trace!(("Failed to load benchmark '{name}'"), err));
                    std::process::exit(1);
                },
            },
        };
        let tests: Vec<TestCase> = match crate::parser::parse_tests(&file) {
            Ok(mut res) => {
                if !args.test.is_empty() {
                    res.retain(|test| args.test.contains(&test.id));
                }
                res
            },
            Err(err) => {
                error!("{}", trace!(("Failed to load benchmark '{name}'"), err));
                std::process::exit(1);
            },
        };
        if !graph.edges.values().any(|e| e.cost > 0.0) {
            warn!("Benchmark '{name}' does not have any cost associated with the links (will assume '1.0' per hop)");
            debug!("Re-assigning link costs...");
            for edge in graph.edges.values_mut() {
                edge.cost = 1.0;
            }
        }
        info!("Benchmark {} ({} nodes, {} edges, '{}')", name, graph.nodes.len(), graph.edges.len(), file.display());



        // Now run some routing algorithm on all tests
        let mut results: HashMap<&str, HashMap<Pipeline, Duration>> = HashMap::new();
        for (i, test) in tests.iter().enumerate() {
            // Benchmark the test
            let mut min_cost: Vec<Option<(String, f64)>> = vec![None; test.k];
            for pip in &pipelines {
                debug!("Benchmarking {} for test '{}' ({}/{})...", pip.name, test.id, i + 1, tests.len());
                let mut g: Graph = graph.clone();
                let start: Instant = Instant::now();
                let (paths, time): (Option<Vec<OwnedPath>>, Duration) =
                    match pip.k_shortest(&mut g, test.source.as_str(), test.target.as_str(), test.k) {
                        Ok(paths) => {
                            let time: Duration = start.elapsed();
                            (paths, time)
                        },
                        Err(err) => {
                            error!("{}", trace!(("Failed to run pipeline '{}'", pip.name), err));
                            std::process::exit(1);
                        },
                    };
                results.entry(test.id.as_str()).or_default().insert(pip.clone(), time);

                // Verify correctness of the paths
                for (i, path) in paths.into_iter().flat_map(Vec::into_iter).enumerate() {
                    // Ensure all entries are connected
                    'hops: for i in 1..path.hops.len() {
                        let n1: &str = path.hops[i - 1].0.as_str();
                        let n2: &str = path.hops[i].0.as_str();
                        for edge in graph.edges.values() {
                            if (edge.left.as_str() == n1 && edge.right.as_str() == n2) || (edge.left.as_str() == n2 && edge.right.as_str() == n1) {
                                continue 'hops;
                            }
                        }
                        panic!("Benchmark '{}' failed for {}: not all paths are connected\n\nPath: {:?}", test.id, pip.name, path);
                    }

                    // Ensure the path connects the test's endpoints
                    if path.hops.first().unwrap().0.as_str() != test.source.as_str() {
                        panic!(
                            "Benchmark '{}' failed for {}: path doesn't start at test source ({})\n\nPath: {:?}",
                            test.id, pip.name, test.source, path
                        );
                    }
                    if path.hops.last().unwrap().0.as_str() != test.target.as_str() {
                        panic!(
                            "Benchmark '{}' failed for {}: path doesn't start at test target ({})\n\nPath: {:?}",
                            test.id, pip.name, test.target, path
                        );
                    }

                    // Check whether the test agrees with the minimum
                    if let Some(prev) = &min_cost[i] {
                        if path.cost() != prev.1 {
                            panic!(
                                "Benchmark '{}' failed for {}: path not shortest (got {}, previous alg got {})\n\nPath:\n{}\n\nPrev path:\n{}\n",
                                test.id,
                                pip.name,
                                path.cost(),
                                prev.1,
                                path,
                                prev.0,
                            );
                        }
                    } else {
                        min_cost[i] = Some((path.to_string(), path.cost()));
                    }
                }
            }
        }

        // Format the results in some nice table
        if !args.csv {
            let mut table = Table::new();
            table.set_header(
                ["Benchmark".to_string(), "Executed test".to_string()]
                    .into_iter()
                    .chain(pipelines.iter().map(|p| format!("{} duration (ms)", p.name))),
            );
            for (test, times) in results {
                table.add_row(
                    [name.to_string(), test.to_string()]
                        .into_iter()
                        .chain(pipelines.iter().map(|p| ((times.get(p).unwrap().as_nanos() as f64) / 1000000.0).to_string())),
                );
            }
            println!("{table}");
        } else {
            // Print the header
            if first {
                print!("Benchmark,Executed test");
                for pip in pipelines.iter() {
                    print!(",{} duration (ms)", pip.name);
                }
                println!();
            }

            // Print the rows
            for (test, times) in results {
                print!("{name},{test}");
                for time in pipelines.iter().map(|p| ((times.get(p).unwrap().as_nanos() as f64) / 1000000.0)) {
                    print!(",{time}");
                }
                println!();
            }
        }

        // OK, did the first one
        first = false;
    }
}
