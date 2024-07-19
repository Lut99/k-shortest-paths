//  RENDER.rs
//    by Lut99
//
//  Created:
//    19 Jul 2024, 00:55:15
//  Last edited:
//    19 Jul 2024, 02:42:10
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the actual renderer to write a [`Graph`] to an image.
//

use image::{Rgba, RgbaImage};
use ksp::Graph;


/***** HELPER FUNCTIONS *****/
/// Scales a given pair of coordinates to pixels.
///
/// # Arguments
/// - `pos`: The coordinates to scale.
/// - `boundaries`: The logical size of the world to scale. Given as two points of a rectangle.
/// - `dims`: The pixel dimensions of the image.
///
/// # Returns
/// A new pair of a (width, height) describing the pixel equivalent.
fn logic_to_pixels(pos: (f64, f64), boundaries: ((f64, f64), (f64, f64)), dims: (u32, u32)) -> (u32, u32) {
    // Scale the positions to ratios over the world
    let pos: (f64, f64) =
        ((pos.0 - boundaries.0.0) / (boundaries.1.0 - boundaries.0.0), (pos.1 - boundaries.0.1) / (boundaries.1.1 - boundaries.0.1));

    // Then discretize
    (((pos.0 * (dims.0 as f64)) + 0.5) as u32, ((pos.1 * (dims.1 as f64)) + 0.5) as u32)
}

/// Draws a line between two coordinates on the image.
///
/// # Arguments
/// - `img`: The [`RgbaImage`] to draw to.
/// - `pos1`: The first pair of coordinates.
/// - `pos2`: The second pair of coordinates.
fn draw_line(img: &mut RgbaImage, pos1: (u32, u32), pos2: (u32, u32)) {
    let (x1, y1): (f64, f64) = (pos1.0 as f64, pos1.1 as f64);
    let (x2, y2): (f64, f64) = (pos2.0 as f64, pos2.1 as f64);

    // Ensure the line isn't vertical
    if pos1.0 == pos2.0 {
        // It is; simply draw down
        for y in std::cmp::min(pos1.1, pos2.1)..std::cmp::max(pos1.1, pos2.1) {
            img[(pos1.0, y)] = Rgba([255, 0, 0, 255]);
        }
        return;
    }

    // Find the line slope and then the formula of it as ax + by + c = 0
    let a: f64 = (y2 - y1) / (x2 - x1);
    let b: f64 = y1 - a * x1;
    let (a, b, c): (f64, f64, f64) = (-a, 1.0, -b);
    let ab2: f64 = (a * a + b * b).sqrt();

    // Create a bounding box around the positions
    let bb: ((u32, u32), (u32, u32)) =
        ((std::cmp::min(pos1.0, pos2.0), std::cmp::min(pos1.1, pos2.1)), (std::cmp::max(pos1.0, pos2.0), std::cmp::max(pos1.1, pos2.1)));

    // Now for all the pixels in the bounding box, colour those within the line
    for y in bb.0.1..=bb.1.1 {
        for x in bb.0.0..=bb.1.0 {
            let d: f64 = (a * x as f64 + b * y as f64 + c).abs() / ab2;

            // Color the pixel if it's within the line
            if d <= 1.0 {
                img[(x, y)] = Rgba([255, 0, 0, 255]);
            }
        }
    }
}

/// Draws a point at a coordinate on the image.
///
/// # Arguments
/// - `img`: The [`RgbaImage`] to draw to.
/// - `pos`: The coordinate to draw the point on.
fn draw_point(img: &mut RgbaImage, pos: (u32, u32)) {
    // Draw in a circle on the image
    for y in pos.1 - 5..pos.1 + 5 {
        for x in pos.0 - 5..pos.0 + 5 {
            let dx: f64 = pos.0 as f64 - x as f64;
            let dy: f64 = pos.1 as f64 - y as f64;
            let r: f64 = (dx * dx + dy * dy).sqrt();
            if r <= 5.0 {
                img[(x, y)] = Rgba([255, 0, 0, 255]);
            }
        }
    }
}





/***** AUXILLARY *****/
/// Defines additional options for rendering.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// The width & height of the resulting image.
    pub dims: (u32, u32),
}
impl Default for Options {
    #[inline]
    fn default() -> Self { Self { dims: (800, 600) } }
}





/***** LIBRARY *****/
/// Renders a given [`Graph`] to an image.
///
/// # Arguments
/// - `graph`: The graph to render.
/// - `opts`: An [`Options`] struct used to configure rendering.
///
/// # Returns
/// A raw [`RgbaImage`] containing the rendered graph.
pub fn render_graph(graph: &Graph, opts: Options) -> RgbaImage {
    // Find the logical boundaries in the graph
    let mut boundaries: (Option<f64>, Option<f64>, Option<f64>, Option<f64>) = (None, None, None, None);
    for node in graph.nodes.values() {
        if node.pos.0 < boundaries.0.unwrap_or(f64::INFINITY) {
            boundaries.0 = Some(node.pos.0);
        }
        if node.pos.1 < boundaries.1.unwrap_or(f64::INFINITY) {
            boundaries.1 = Some(node.pos.1);
        }
        if node.pos.0 > boundaries.2.unwrap_or(-f64::INFINITY) {
            boundaries.2 = Some(node.pos.0);
        }
        if node.pos.1 > boundaries.3.unwrap_or(-f64::INFINITY) {
            boundaries.3 = Some(node.pos.1);
        }
    }
    let boundaries: ((f64, f64), (f64, f64)) = match boundaries {
        // Return the found boundaries plus some 1/10th of the area extra for prettiness
        (Some(x1), Some(y1), Some(x2), Some(y2)) => ((x1 - (x2 - x1) / 10.0, y1 - (y2 - y1) / 10.0), (x2 + (x2 - x1) / 10.0, y2 + (y2 - y1) / 10.0)),
        _ => unimplemented!(),
    };

    // Create a white image to draw on
    let mut img = RgbaImage::new(opts.dims.0, opts.dims.1);
    img.fill(255);

    // Draw all edges first
    for edge in graph.edges.values() {
        // Get the two points in pixels
        let pos1: (u32, u32) = logic_to_pixels(graph.nodes.get(&edge.left).unwrap().pos, boundaries, opts.dims);
        let pos2: (u32, u32) = logic_to_pixels(graph.nodes.get(&edge.right).unwrap().pos, boundaries, opts.dims);

        // Draw a line between them
        draw_line(&mut img, pos1, pos2);
    }

    // Draw the nodes
    for node in graph.nodes.values() {
        draw_point(&mut img, logic_to_pixels(node.pos, boundaries, opts.dims));
    }

    // Done
    img
}
