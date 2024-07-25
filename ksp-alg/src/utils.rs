//  UTILS.rs
//    by Lut99
//
//  Created:
//    20 Jul 2024, 01:05:09
//  Last edited:
//    25 Jul 2024, 20:35:34
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines utilities for use in tests.
//

#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
use error_trace::trace;
#[cfg(test)]
use ksp_graph::Graph;


/***** LIBRARY *****/
/// Implements a parsable options enum.
macro_rules! parsable_enum_impl {
    ($(#[$outer:meta])*pub enum $name:ident { $($(#[doc = $docs:literal])* $(#[cfg($cfgs:meta)])* $variants:ident $(($field:ty))? { $($strings:literal => $gen:expr),+ $(,)? }),* $(,)? }) => {
        ::paste::paste! {
            #[doc = concat!("Defines errors occurring from parsing [`", stringify!($name), "`] from strings.")]
            #[derive(::std::fmt::Debug)]
            pub struct [<Unknown $name Error>] {
                /// The string that was unknown
                pub unknown: String,
            }
            impl ::std::fmt::Display for [<Unknown $name Error>] {
                #[inline]
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, concat!("Unknown ", stringify!($name), " '{}'"), self.unknown)
                }
            }
            impl ::std::error::Error for [<Unknown $name Error>] {}
        }


        $(#[$outer])*
        pub enum $name {
            $($(#[doc = $docs])* $(#[cfg($cfgs)])* $variants $(($field))?,)*
        }
        impl $name {
            /// Returns all variants in this enum.
            ///
            /// # Returns
            /// A static slice of all variants.
            #[inline]
            pub const fn all(&self) -> &'static [Self] {
                &[$($($gen,)+)*]
            }
        }
        ::paste::paste! {
            impl ::std::str::FromStr for $name {
                type Err = [<Unknown $name Error>];

                #[inline]
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        $($($strings => Ok($gen),)+)*
                        raw => Err([<Unknown $name Error>] { unknown: raw.into() }),
                    }
                }
            }
        }
    };
}
pub(crate) use parsable_enum_impl;



/// Loads a test graph with a given name.
///
/// # Arguments
/// - `name`: The name of the file to load. Doesn't need to include `.json` (but it can).
///
/// # Returns
/// A loaded [`Graph`].
///
/// # Panics
/// This function panics if it failed to load the given file.
#[cfg(test)]
pub fn load_graph(name: impl AsRef<str>) -> Graph {
    let name: &str = name.as_ref();

    // Check if the file exists without mods
    let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("tests").join(name);
    if !path.exists() {
        path.set_file_name(format!("{name}.json"));
    }

    // OK try to do it
    match ksp_graph::json::parse(&path) {
        Ok(g) => g,
        Err(err) => panic!("{}", trace!(("Failed to load graph file '{}'", path.display()), err)),
    }
}

/// Loads a benchmark graph with a given name.
///
/// # Arguments
/// - `name`: The name of the file to load. Doesn't need to include `.xml` (but it can).
///
/// # Returns
/// A loaded [`Graph`].
///
/// # Panics
/// This function panics if it failed to load the given file.
#[cfg(test)]
pub fn load_bench(name: impl AsRef<str>) -> Graph {
    let name: &str = name.as_ref();

    // Check if the file exists without mods
    let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("benchmarks").join(name);
    if !path.exists() {
        path.set_file_name(format!("{name}.xml"));
    }

    // OK try to do it
    match ksp_graph::sndlib_xml::parse(&path) {
        Ok(g) => g,
        Err(err) => panic!("{}", trace!(("Failed to load graph file '{}'", path.display()), err)),
    }
}
