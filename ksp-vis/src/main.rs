//  MAIN.rs
//    by Lut99
//
//  Created:
//    16 Jul 2024, 01:44:40
//  Last edited:
//    19 Jul 2024, 02:40:32
//  Auto updated?
//    Yes
//
//  Description:
//!   Entrypoint for the `visualize` binary.
//

use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

use arrayvec::ArrayString;
use error_trace::trace;
use image::{ImageFormat, RgbaImage};
use ksp::{Edge, Graph, Node};
use ksp_vis::render::{render_graph, Options};
use log::error;


/***** ENTRYPOINT *****/
fn main() {
    let g = Graph {
        nodes: HashMap::from([
            (ArrayString::from("Amsterdam").unwrap(), Node { id: ArrayString::from("Amsterdam").unwrap(), pos: (52.3673, 4.9041) }),
            (ArrayString::from("Berlin").unwrap(), Node { id: ArrayString::from("Berlin").unwrap(), pos: (52.5200, 13.4050) }),
            (ArrayString::from("Chicago").unwrap(), Node { id: ArrayString::from("Chicago").unwrap(), pos: (41.8781, 87.6298) }),
            (ArrayString::from("Dorchester").unwrap(), Node { id: ArrayString::from("Dorchester").unwrap(), pos: (50.7112, 2.4412) }),
            (ArrayString::from("Edinburgh").unwrap(), Node { id: ArrayString::from("Edinburgh").unwrap(), pos: (55.9533, 3.1883) }),
        ]),
        edges: HashMap::from([
            (ArrayString::from("Amsterdam-Berlin").unwrap(), Edge {
                id:    ArrayString::from("Amsterdam-Berlin").unwrap(),
                left:  ArrayString::from("Amsterdam").unwrap(),
                right: ArrayString::from("Berlin").unwrap(),
                cost:  577.34,
            }),
            (ArrayString::from("Amsterdam-Dorchester").unwrap(), Edge {
                id:    ArrayString::from("Amsterdam-Dorchester").unwrap(),
                left:  ArrayString::from("Amsterdam").unwrap(),
                right: ArrayString::from("Dorchester").unwrap(),
                cost:  540.86,
            }),
            (ArrayString::from("Amsterdam-Edinburgh").unwrap(), Edge {
                id:    ArrayString::from("Amsterdam-Edinburgh").unwrap(),
                left:  ArrayString::from("Amsterdam").unwrap(),
                right: ArrayString::from("Edinburgh").unwrap(),
                cost:  660.68,
            }),
            (ArrayString::from("Dorchester-Edinburgh").unwrap(), Edge {
                id:    ArrayString::from("Dorchester-Edinburgh").unwrap(),
                left:  ArrayString::from("Dorchester").unwrap(),
                right: ArrayString::from("Edinburgh").unwrap(),
                cost:  589.23,
            }),
            (ArrayString::from("Chicago-Dorchester").unwrap(), Edge {
                id:    ArrayString::from("Chicago-Dorchester").unwrap(),
                left:  ArrayString::from("Chicago").unwrap(),
                right: ArrayString::from("Dorchester").unwrap(),
                cost:  6249.15,
            }),
        ]),
    };

    // Render
    let img: RgbaImage = render_graph(&g, Options::default());
    let mut flipped: RgbaImage = img.clone();
    for y in 0..img.height() {
        for x in 0..img.width() {
            flipped[(x, img.height() - 1 - y)] = img[(x, y)];
        }
    }

    // Write the image
    let path: PathBuf = PathBuf::from("./output.png");
    match File::create(&path) {
        Ok(mut handle) => {
            if let Err(err) = flipped.write_to(&mut handle, ImageFormat::Png) {
                error!("{}", trace!(("Failed to write to output image '{}'", path.display()), err));
                std::process::exit(1);
            }
        },
        Err(err) => {
            error!("{}", trace!(("Failed to create output image '{}'", path.display()), err));
            std::process::exit(1);
        },
    }
}
