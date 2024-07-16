//  MAIN.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 00:09:40
//  Last edited:
//    16 Jul 2024, 20:39:03
//  Auto updated?
//    Yes
//
//  Description:
//!   Entrypoint for the `benchmark`-binary.
//

use std::collections::HashMap;
use std::fs::{self, DirEntry, ReadDir};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use clap::Parser;
use comfy_table::Table;
use error_trace::trace;
use humanlog::{DebugMode, HumanLogger};
use ksp::graph::Graph;
use ksp::{Algorithm, Path, Routing};
use ksp_bench::parser::{self};
use ksp_bench::tests::TestCase;
use log::{debug, error, info, warn};


/***** ARGUMENTS *****/
#[derive(Clone, Debug, Parser)]
struct Arguments {
    /// Whether to run with additional log statements.
    #[clap(long, global = true, help = "If given, shows DEBUG- and INFO-level log statements.")]
    debug: bool,
    /// Whether to run with maximum log statements.
    #[clap(long, global = true, help = "If given, shows TRACE-level log statements. Implies '--debug'.")]
    trace: bool,

    /// The algorithm to benchmark.
    #[clap(name = "ALGORITHMS", help = "The algorithms to benchmark. Give multiple for side-by-side comparisons.")]
    algorithms: Vec<Algorithm>,

    /// Any specific benchmarks to run.
    #[clap(
        short,
        long,
        help = "If given, does not run all benchmarks in the '--benchmark-dir', but instead only the ones with the given name. Adding '.xml' is \
                optional."
    )]
    benchmark:     Vec<String>,
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



    // Resolve to a list of benchmark files
    let mut files: Vec<(String, PathBuf)> = Vec::new();
    if !args.benchmark.is_empty() {
        // Resolve the entries
        for entry in args.benchmark {
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
            files.push((entry, path));
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
            if !entry_path.ends_with("xml") {
                debug!("Skipping entry '{}' as it does not end in '.xml'", entry_path.display());
            }

            // Add it
            files.push((entry_path.file_stem().map(|n| n.to_string_lossy().into()).unwrap_or(i.to_string()), entry_path));
        }
    }

    // Run them
    debug!("Running {} benchmark(s)", files.len());
    let mut first: bool = true;
    for (name, file) in files {
        // Open the file
        let (mut graph, tests): (Graph, Vec<TestCase>) = match parser::parse_sndlib_xml(&file) {
            Ok(res) => res,
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
        let mut results: HashMap<&str, HashMap<Algorithm, Duration>> = HashMap::new();
        for (i, test) in tests.iter().enumerate() {
            debug!("Benchmarking for test '{}' ({}/{})...", test.id, i + 1, tests.len());

            // Benchmark the test
            for alg in &args.algorithms {
                match alg {
                    Algorithm::Vanilla => {
                        // Run the test and time it
                        let start = Instant::now();
                        let _paths: Vec<Path> = ksp::vanilla::VanillaKSP.k_shortest_paths(&graph, &test.source, &test.target, test.k);
                        let time = start.elapsed();

                        // Store that
                        results.entry(&test.id).or_default().insert(Algorithm::Vanilla, time);
                    },
                }
            }
        }

        // Format the results in some nice table
        if !args.csv {
            let mut table = Table::new();
            table.set_header(
                ["Benchmark".to_string(), "Executed test".to_string()]
                    .into_iter()
                    .chain(results.values().next().into_iter().flat_map(|t| t.keys().map(|a| format!("{a:?} duration (ms)")))),
            );
            for (test, times) in results {
                table.add_row(
                    [name.to_string(), test.to_string()]
                        .into_iter()
                        .chain(times.into_values().map(|t| ((t.as_nanos() as f64) / 1000000.0).to_string())),
                );
            }
            println!("{table}");
        } else {
            // Print the header
            if first {
                print!("Benchmark,Executed test");
                for alg in results.values().next().into_iter().flat_map(|t| t.keys().map(|a| format!("{a:?} duration (ms)"))) {
                    print!(",{alg}");
                }
                println!();
            }

            // Print the rows
            for (test, times) in results {
                print!("{name},{test}");
                for time in times.into_values().map(|t| (t.as_nanos() as f64) / 1000000.0) {
                    print!(",{time}");
                }
                println!();
            }
        }

        // OK, did the first one
        first = false;
    }
}
