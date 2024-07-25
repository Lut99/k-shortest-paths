//  LIB.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 00:06:19
//  Last edited:
//    25 Jul 2024, 20:21:59
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the different implementations of K-Shortest Path (KSP)
//!   algorithms.
//

// Declare modules
pub mod dist;
pub mod ksp;
pub mod path;
pub mod sssp;
pub mod trans;
pub mod utils;

// use std::error::Error;
// use std::fmt::{Display, Formatter, Result as FResult};
// use std::str::FromStr;
// use std::time::{Duration, Instant};

// use ksp_graph::Graph;
// use prep::peek::PeekPreprocess;
// use prep::Step;
// use sssp::dijkstra::DijkstraSSSP;
// use sssp::profiled::ProfilingSSSP;
// use sssp::Sssp;
// use yen::YenKSP;

// Use some of it in this namespace
pub use crate::ksp::*;
pub use crate::path::*;


// /***** ERRORS *****/
// /// Failed to parse a [`Pipeline`] from a string.
// #[derive(Debug)]
// pub enum PipelineParseError {
//     /// An algorithm requiring SSSP was given without.
//     AlgMissingSSSP { alg: Algorithm },
//     /// Failed to parse an algorithm.
//     IllegalAlgorithm { raw: String, err: ksp::UnknownAlgorithmError },
//     /// Failed to parse the SSSP algorithm.
//     IllegalSssp { raw: String, err: sssp::UnknownSsspError },
//     /// Failed to parse a preprocessing step.
//     IllegalStep { raw: String, err: prep::UnknownStepError },
//     /// Missing the closing parenthesis wrapping the SSSP.
//     MissingClosingDelim { raw: String },
//     /// A preprocessing step requiring SSSP was given without.
//     StepMissingSSSP { step: Step },
// }
// impl Display for PipelineParseError {
//     #[inline]
//     fn fmt(&self, f: &mut Formatter) -> FResult {
//         use PipelineParseError::*;
//         match self {
//             AlgMissingSSSP { alg } => write!(f, "Algorithm '{alg:?}' requires an SSSP algorithm to be defined"),
//             IllegalAlgorithm { raw, .. } => write!(f, "Failed to parse '{raw}' as an algorithm"),
//             IllegalSssp { raw, .. } => write!(f, "Failed to parse '{raw}' as an SSSP algorithm"),
//             IllegalStep { raw, .. } => write!(f, "Failed to parse '{raw}' as a preprocessing step"),
//             MissingClosingDelim { raw } => write!(f, "Missing closing delimiter '>' in '{raw}'"),
//             StepMissingSSSP { step } => write!(f, "Preprocessing step '{step:?}' requires an SSSP algorithm to be defined"),
//         }
//     }
// }
// impl Error for PipelineParseError {
//     #[inline]
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         use PipelineParseError::*;
//         match self {
//             AlgMissingSSSP { .. } => None,
//             IllegalAlgorithm { err, .. } => Some(err),
//             IllegalSssp { err, .. } => Some(err),
//             IllegalStep { err, .. } => Some(err),
//             MissingClosingDelim { .. } => None,
//             StepMissingSSSP { .. } => None,
//         }
//     }
// }





// /***** HELPERS *****/
// /// Defines profile timings of a [`Pipeline`]-run.
// #[derive(Clone, Debug)]
// pub struct PipelineProfile {
//     /// The amount of time each step took.
//     pub prep: Vec<Duration>,
//     /// The time the main algorithm took.
//     pub alg:  Duration,
//     /// The timings for all SSSP calls, if any.
//     pub sssp: Vec<Duration>,
// }





// /***** LIBRARY *****/
// /// Defines a full chain that configures which KSP algorithm is run and how.
// #[derive(Clone, Debug, Eq, Hash, PartialEq)]
// pub struct Pipeline {
//     /// Preprocess steps to take.
//     prep: Vec<(prep::Step, Option<sssp::Sssp>)>,
//     /// The algorithm to execute.
//     alg:  Algorithm,
//     /// Which SSSP algorithm to use if applicable.
//     sssp: Option<sssp::Sssp>,
// }
// impl Pipeline {
//     /// Computes the K-Shortest Path algorithm as defined by this [`Pipeline`].
//     ///
//     /// # Arguments
//     /// - `graph`: The [`Graph`] to find in.
//     /// - `src`: The source node to find a path from.
//     /// - `dst`: The destination node to find a path to.
//     /// - `k`: The number of paths to find.
//     ///
//     /// # Returns
//     /// A pair of the list of the shortest paths found and a [`PipelineProfile`] detailling how long every step took.
//     ///
//     /// The path list is at most `k` elements long.
//     ///
//     /// # Panics
//     /// This function is allowed to panic if the given `src` or `dst` are not in the given `graph`.
//     #[inline]
//     pub fn k_shortest_paths_profiled<'g>(&self, graph: &'g mut Graph, src: &str, dst: &str, k: usize) -> (Vec<Path<'g>>, PipelineProfile) {
//         // First, pre-process the graph
//         let mut prep_timings: Vec<Duration> = Vec::with_capacity(self.prep.len());
//         for (p, s) in &self.prep {
//             use prep::PreprocessStep as _;
//             match (p, s) {
//                 (Step::Peek, Some(Sssp::Dijkstra)) => {
//                     // Initialize the algorithm
//                     let mut step = PeekPreprocess::new(DijkstraSSSP);

//                     // Run & measure it
//                     let start: Instant = Instant::now();
//                     step.preprocess(graph, src, dst, k);
//                     prep_timings.push(start.elapsed());
//                 },
//                 (Step::Peek, None) => panic!("Cannot execute PeeK without an SSSP defined"),
//             }
//         }

//         // Run the appropriate KSP algorithm
//         match (&self.alg, &self.sssp) {
//             (Algorithm::Wikipedia, _) => {
//                 // Run the alg with timings
//                 let start: Instant = Instant::now();
//                 let paths: Vec<Path<'g>> = ksp::wikipedia::WikipediaKSP.k_shortest_paths(graph, src, dst, k);
//                 let time: Duration = start.elapsed();

//                 // Return the full profile
//                 (paths, PipelineProfile { prep: prep_timings, alg: time, sssp: vec![] })
//             },
//             (Algorithm::Yen, Some(sssp::Sssp::Dijkstra)) => {
//                 // Prepare the algorithm with a wrapped SSSP profiler
//                 let mut alg = YenKSP::new(ProfilingSSSP::new(sssp::dijkstra::DijkstraSSSP));

//                 // Run the alg with timings
//                 let start: Instant = Instant::now();
//                 let paths: Vec<Path<'g>> = alg.k_shortest_paths(graph, src, dst, k);
//                 let time: Duration = start.elapsed();

//                 // Return the full profile
//                 (paths, PipelineProfile { prep: prep_timings, alg: time, sssp: alg.sssp.timings })
//             },
//             (Algorithm::Yen, None) => panic!("Cannot run Yen without SSSP defined"),
//         }
//     }
// }
// impl Display for Pipeline {
//     #[inline]
//     fn fmt(&self, f: &mut Formatter) -> FResult {
//         for (step, sssp) in &self.prep {
//             write!(f, "{step:?}")?;
//             if let Some(sssp) = sssp {
//                 write!(f, "<{sssp:?}>")?;
//             }
//             write!(f, "->")?;
//         }
//         write!(f, "{:?}", self.alg)?;
//         if let Some(sssp) = &self.sssp {
//             write!(f, "<{sssp:?}>")?;
//         }
//         Ok(())
//     }
// }
// impl FromStr for Pipeline {
//     type Err = PipelineParseError;

//     #[inline]
//     fn from_str(mut s: &str) -> Result<Self, Self::Err> {
//         // First, parse steps
//         let mut prep: Vec<(Step, Option<Sssp>)> = Vec::new();
//         while let Some(pos) = s.find("->") {
//             // Get a chunk suffixed by '->'
//             let step: &str = &s[..pos];
//             s = &s[pos + 2..];

//             // See if we're talking about an SSSP addition or not
//             match s.find('<') {
//                 Some(pos) => {
//                     // Split it
//                     let step: &str = &s[..pos];
//                     s = &s[pos + 1..];

//                     // Ensure 's' now ends with a closing parenthesis
//                     if &s[s.len() - 1..] != ">" {
//                         return Err(PipelineParseError::MissingClosingDelim { raw: s.into() });
//                     }
//                     s = &s[..s.len() - 1];

//                     // Parse the step
//                     let step: Step = match Step::from_str(step) {
//                         Ok(step) => step,
//                         Err(err) => return Err(PipelineParseError::IllegalStep { raw: step.into(), err }),
//                     };

//                     // Parse the SSSP
//                     match Sssp::from_str(s) {
//                         Ok(sssp) => prep.push((step, Some(sssp))),
//                         Err(err) => return Err(PipelineParseError::IllegalSssp { raw: s.into(), err }),
//                     }
//                 },

//                 None => {
//                     // Attempt to parse the step as a Step, then
//                     match Step::from_str(step) {
//                         Ok(step) => {
//                             // Ensure this step doesn't need it, either
//                             if !step.needs_sssp() { prep.push((step, None)) } else { return Err(PipelineParseError::StepMissingSSSP { step }) }
//                         },
//                         Err(err) => return Err(PipelineParseError::IllegalStep { raw: step.into(), err }),
//                     }
//                 },
//             }
//         }

//         // See if we need to split further
//         match s.find('<') {
//             Some(pos) => {
//                 // Split into algorithm and sssp
//                 let alg: &str = &s[..pos];
//                 s = &s[pos + 1..];

//                 // Ensure 's' now ends with a closing parenthesis
//                 if &s[s.len() - 1..] != ">" {
//                     return Err(PipelineParseError::MissingClosingDelim { raw: s.into() });
//                 }
//                 s = &s[..s.len() - 1];

//                 // Parse the algorithm
//                 let alg: Algorithm = match Algorithm::from_str(alg) {
//                     Ok(alg) => alg,
//                     Err(err) => return Err(PipelineParseError::IllegalAlgorithm { raw: alg.into(), err }),
//                 };

//                 // Parse the SSSP
//                 match Sssp::from_str(s) {
//                     Ok(sssp) => Ok(Self { prep, alg, sssp: Some(sssp) }),
//                     Err(err) => Err(PipelineParseError::IllegalSssp { raw: s.into(), err }),
//                 }
//             },

//             None => {
//                 // The remainder should be the algorithm
//                 match Algorithm::from_str(s) {
//                     Ok(alg) => {
//                         // Ensure SSSP is given if it's needed
//                         if !alg.needs_sssp() { Ok(Self { prep, alg, sssp: None }) } else { Err(PipelineParseError::AlgMissingSSSP { alg }) }
//                     },
//                     Err(err) => Err(PipelineParseError::IllegalAlgorithm { raw: s.into(), err }),
//                 }
//             },
//         }
//     }
// }
