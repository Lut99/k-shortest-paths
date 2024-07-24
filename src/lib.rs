//  LIB.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 00:06:19
//  Last edited:
//    24 Jul 2024, 23:33:03
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the different implementations of K-Shortest Path (KSP)
//!   algorithms.
//

// Declare modules
pub mod ksp;
pub mod path;
pub mod prep;
pub mod sssp;
#[cfg(test)]
pub mod utils;

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::str::FromStr;
use std::time::{Duration, Instant};

use ksp_graph::Graph;
use sssp::profiled::ProfilingSSSP;
use sssp::Sssp;

// Use some of it in this namespace
pub use crate::ksp::*;
pub use crate::path::*;


/***** ERRORS *****/
/// Failed to parse a [`Pipeline`] from a string.
#[derive(Debug)]
pub enum PipelineParseError {
    /// Failed to parse an algorithm.
    IllegalAlgorithm { raw: String, err: ksp::UnknownAlgorithmError },
    /// Failed to parse the SSSP algorithm.
    IllegalSssp { raw: String, err: sssp::UnknownSsspError },
    /// Failed to parse a preprocessing step.
    IllegalStep { raw: String, err: prep::UnknownStepError },
    /// Missing the closing parenthesis wrapping the SSSP.
    MissingClosingDelim { raw: String },
    /// An algorithm requiring SSSP was given without.
    MissingSSSP { alg: Algorithm },
}
impl Display for PipelineParseError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FResult {
        use PipelineParseError::*;
        match self {
            IllegalAlgorithm { raw, .. } => write!(f, "Failed to parse '{raw}' as an algorithm"),
            IllegalSssp { raw, .. } => write!(f, "Failed to parse '{raw}' as an SSSP algorithm"),
            IllegalStep { raw, .. } => write!(f, "Failed to parse '{raw}' as a preprocessing step"),
            MissingClosingDelim { raw } => write!(f, "Missing closing delimiter '>' in '{raw}'"),
            MissingSSSP { alg } => write!(f, "Algorithm '{alg:?}' requires an SSSP algorithm to be defined"),
        }
    }
}
impl Error for PipelineParseError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use PipelineParseError::*;
        match self {
            IllegalAlgorithm { err, .. } => Some(err),
            IllegalSssp { err, .. } => Some(err),
            IllegalStep { err, .. } => Some(err),
            MissingClosingDelim { .. } => None,
            MissingSSSP { .. } => None,
        }
    }
}





/***** HELPERS *****/
/// Defines profile timings of a [`Pipeline`]-run.
#[derive(Clone, Debug)]
pub struct PipelineProfile {
    /// The amount of time each step took.
    pub prep: Vec<Duration>,
    /// The time the main algorithm took.
    pub alg:  Duration,
    /// The timings for all SSSP calls, if any.
    pub sssp: Vec<Duration>,
}





/***** LIBRARY *****/
/// Defines a full chain that configures which KSP algorithm is run and how.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pipeline {
    /// Preprocess steps to take.
    prep: Vec<prep::Step>,
    /// The algorithm to execute.
    alg:  Algorithm,
    /// Which SSSP algorithm to use if applicable.
    sssp: Option<sssp::Sssp>,
}
impl Pipeline {
    /// Computes the K-Shortest Path algorithm as defined by this [`Pipeline`].
    ///
    /// # Arguments
    /// - `graph`: The [`Graph`] to find in.
    /// - `src`: The source node to find a path from.
    /// - `dst`: The destination node to find a path to.
    /// - `k`: The number of paths to find.
    ///
    /// # Returns
    /// A pair of the list of the shortest paths found and a [`PipelineProfile`] detailling how long every step took.
    ///
    /// The path list is at most `k` elements long.
    ///
    /// # Panics
    /// This function is allowed to panic if the given `src` or `dst` are not in the given `graph`.
    #[inline]
    pub fn k_shortest_paths_profiled<'g>(&self, graph: &'g mut Graph, src: &str, dst: &str, k: usize) -> (Vec<Path<'g>>, PipelineProfile) {
        // First, pre-process the graph
        let mut prep_timings: Vec<Duration> = Vec::with_capacity(self.prep.len());
        for p in &self.prep {
            use prep::PreprocessStep as _;
            match p {
                prep::Step::Peek => {
                    let start: Instant = Instant::now();
                    prep::peek::PeekPreprocess::preprocess(graph, src, dst, k);
                    prep_timings.push(start.elapsed());
                },
            }
        }

        // Run the appropriate KSP algorithm
        match (&self.alg, &self.sssp) {
            (Algorithm::Wikipedia, _) => {
                // Run the alg with timings
                let start: Instant = Instant::now();
                let paths: Vec<Path<'g>> = ksp::wikipedia::WikipediaKSP.k_shortest_paths(graph, src, dst, k);
                let time: Duration = start.elapsed();

                // Return the full profile
                (paths, PipelineProfile { prep: prep_timings, alg: time, sssp: vec![] })
            },
            (Algorithm::Yen, Some(sssp::Sssp::Dijkstra)) => {
                // Prepare the wrapped SSSP profiler
                let mut sssp: ProfilingSSSP<sssp::dijkstra::DijkstraSSSP> = ProfilingSSSP::new(sssp::dijkstra::DijkstraSSSP);

                // Run the alg with timings
                let start: Instant = Instant::now();
                let paths: Vec<Path<'g>> = ksp::yen::YenKSP::new(&mut sssp).k_shortest_paths(graph, src, dst, k);
                let time: Duration = start.elapsed();

                // Return the full profile
                (paths, PipelineProfile { prep: prep_timings, alg: time, sssp: sssp.timings })
            },
            (Algorithm::Yen, None) => panic!("Cannot run Yen without SSSP defined"),
        }
    }
}
impl Display for Pipeline {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FResult {
        for step in &self.prep {
            write!(f, "{step:?}->")?;
        }
        write!(f, "{:?}", self.alg)?;
        if let Some(sssp) = &self.sssp {
            write!(f, "<{sssp:?}>")?;
        }
        Ok(())
    }
}
impl FromStr for Pipeline {
    type Err = PipelineParseError;

    #[inline]
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        // First, parse steps
        let mut prep: Vec<prep::Step> = Vec::new();
        while let Some(pos) = s.find("->") {
            // Get a chunk suffixed by '->'
            let step: &str = &s[..pos];
            s = &s[pos + 2..];

            // Attempt to parse the step as a Step
            match prep::Step::from_str(step) {
                Ok(step) => prep.push(step),
                Err(err) => return Err(PipelineParseError::IllegalStep { raw: step.into(), err }),
            }
        }

        // See if we need to split further
        match s.find('<') {
            Some(pos) => {
                // Split into algorithm and sssp
                let alg: &str = &s[..pos];
                s = &s[pos + 1..];

                // Ensure 's' now ends with a closing parenthesis
                if &s[s.len() - 1..] != ">" {
                    return Err(PipelineParseError::MissingClosingDelim { raw: s.into() });
                }
                s = &s[..s.len() - 1];

                // Parse the algorithm
                let alg: Algorithm = match Algorithm::from_str(alg) {
                    Ok(alg) => alg,
                    Err(err) => return Err(PipelineParseError::IllegalAlgorithm { raw: alg.into(), err }),
                };

                // Parse the SSSP
                match Sssp::from_str(s) {
                    Ok(sssp) => Ok(Self { prep, alg, sssp: Some(sssp) }),
                    Err(err) => Err(PipelineParseError::IllegalSssp { raw: s.into(), err }),
                }
            },

            None => {
                // The remainder should be the algorithm
                match Algorithm::from_str(s) {
                    Ok(alg) => {
                        // Ensure SSSP is given if it's needed
                        if !alg.needs_sssp() { Ok(Self { prep, alg, sssp: None }) } else { Err(PipelineParseError::MissingSSSP { alg }) }
                    },
                    Err(err) => Err(PipelineParseError::IllegalAlgorithm { raw: s.into(), err }),
                }
            },
        }
    }
}
