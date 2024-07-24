//  RENDER.rs
//    by Lut99
//
//  Created:
//    19 Jul 2024, 00:55:15
//  Last edited:
//    25 Jul 2024, 00:09:55
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the actual renderer to write a [`Graph`] to an image.
//

use std::cmp::{max, min};

use image::{GenericImageView, Pixel, Rgb, Rgba, RgbaImage};
use ksp_graph::Graph;
use lazy_static::lazy_static;
use rusttype::{point, Font, PositionedGlyph, Scale, VMetrics};


/***** CONSTANTS *****/
/// The embedded TTF file.
const FONT_RAW: &[u8] = include_bytes!("../assets/OpenSans-Regular.ttf");

lazy_static! {
    /// A parsed variation of the [`FONT_RAW`] font used for [`draw_label()`].
    static ref FONT: Font<'static> = Font::try_from_bytes(FONT_RAW).unwrap_or_else(|| panic!("Failed to construct font"));
    /// The size at which we render text.
    static ref FONT_SIZE: Scale = Scale::uniform(16.0);
}





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

/// Draws a label next to a point on the image.
///
/// Attempts to do some clever placing if at all possible.
///
/// Note that the main rendering algorithm of text is taken from:
/// <https://gitlab.redox-os.org/redox-os/rusttype/-/blob/master/dev/examples/image.rs?ref_type=heads>
///
/// # Arguments
/// - `img`: The [`RgbaImage`] to draw to.
/// - `pos`: The coordinate to draw the point on.
/// - `label`: The label to write.
/// - `bg`: If given, gives the labels a static background colour.
/// - `clever_placement`: If true, then it will attempt to find a best place to display the label _around_ the chosen position. Else, will just place it over the given pos.
fn draw_label(img: &mut RgbaImage, pos: (u32, u32), label: &str, bg: Option<Rgb<u8>>, clever_placement: bool) {
    // Render the text to a smaller image
    let text: RgbaImage = {
        // Find out what the vertical properties are of this font
        let v_metrics: VMetrics = FONT.v_metrics(*FONT_SIZE);

        // Layout the glyphs
        let glyphs: Vec<PositionedGlyph<'static>> = FONT.layout(label, *FONT_SIZE, point(0.0, v_metrics.ascent)).collect();

        // Work out the total layout size
        let (glyphs_width, x_offset): (u32, i32) = {
            let min_x = glyphs.first().map(|g| g.pixel_bounding_box().unwrap().min.x).unwrap();
            let max_x = glyphs.last().map(|g| g.pixel_bounding_box().unwrap().max.x).unwrap();
            ((max_x - min_x) as u32, min_x)
        };
        let glyphs_height: u32 = (v_metrics.ascent - v_metrics.descent).ceil() as u32;

        // Now actually render all those glyphs
        let mut text: RgbaImage = RgbaImage::new(glyphs_width, glyphs_height);
        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                // We draw the glyph pixel-for-pixel
                glyph.draw(|x, y, v| {
                    text.put_pixel(
                        ((x as i32 + bb.min.x) - x_offset) as u32,
                        glyphs_height - 1 - (y + bb.min.y as u32),
                        Rgba([0, 0, 0, (v * 255.0 + 0.5) as u8]),
                    );
                })
            }
        }

        // Trim the top- and bottom layers
        let mut n_top: u32 = 0;
        for mut row in img.rows() {
            if row.any(|p| p.0[3] > 0) {
                break;
            }
            n_top += 1;
        }
        let mut n_bot: u32 = 0;
        for mut row in img.rows().rev() {
            if row.any(|p| p.0[3] > 0) {
                break;
            }
            n_bot += 1;
        }
        text = text.view(0, n_top, text.width(), text.height() - n_top - n_bot).to_image();

        // If there's a background colour, generate that first
        if let Some(color) = bg {
            let color: Rgba<u8> = color.to_rgba();

            // Generate the static background color
            let mut bg: RgbaImage = RgbaImage::new(text.width(), text.height());
            for pix in bg.pixels_mut() {
                *pix = color;
            }

            // Merge the text onto it
            image::imageops::overlay(&mut bg, &text, 0, 0);
            text = bg;
        }

        // Done
        text
    };

    // Define the positions to try
    let posses: &[(((u32, u32), (u32, u32)), bool)] = if clever_placement {
        // Attempt to position it BOTTOM, LEFT, TOP, RIGHT, then BOTTOM but just forcing it
        &[
            (((pos.0 - text.width() / 2, pos.1 - text.height() - 5), (pos.0 + text.width() / 2, pos.1 - 5)), false),
            (((pos.0 - text.width() - 5, pos.1 - text.height() / 2), (pos.0 - 5, pos.1 + text.height() / 2)), false),
            (((pos.0 - text.width() / 2, pos.1 + 5), (pos.0 + text.width() / 2, pos.1 + text.height() + 5)), false),
            (((pos.0 + 5, pos.1 - text.height() / 2), (pos.0 + text.width() + 5, pos.1 + text.height() / 2)), false),
            (((pos.0 - text.width() / 2, pos.1 - text.height() - 5), (pos.0 + text.width() / 2, pos.1 - 5)), true),
        ]
    } else {
        // Just force it on the position itself
        &[(((pos.0 - text.width() / 2, pos.1 - text.height() / 2), (pos.0 + text.width() / 2, pos.1 + text.height() / 2)), true)]
    };

    // Attempt to position the label
    for (bb, force) in posses {
        // See if we're overlapping with anything
        if !force && pos.1 >= 5 + text.height() {
            let mut clear: bool = true;
            for y in bb.0.1..bb.1.1 {
                for x in bb.0.0..bb.1.0 {
                    if text[(x - bb.0.0, y - bb.0.1)].0[3] > 0 && img[(x, y)] != Rgba([255, 255, 255, 255]) {
                        clear = false;
                        break;
                    }
                }
            }
            if !clear {
                continue;
            }
        }

        // If we made it here, we're good to write
        image::imageops::overlay(img, &text, bb.0.0 as i64, bb.0.1 as i64);
        return;
    }

    // Just ignore for now
    unreachable!();
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

        // Annotate the cost
        let bb: ((u32, u32), (u32, u32)) = ((min(pos1.0, pos2.0), min(pos1.1, pos2.1)), (max(pos1.0, pos2.0), max(pos1.1, pos2.1)));
        draw_label(
            &mut img,
            (bb.0.0 + (bb.1.0 - bb.0.0) / 2, bb.0.1 + (bb.1.1 - bb.0.1) / 2),
            &format!("{:.2}", edge.cost),
            Some(Rgb([255, 255, 255])),
            false,
        );
    }

    // Draw the nodes
    for node in graph.nodes.values() {
        draw_point(&mut img, logic_to_pixels(node.pos, boundaries, opts.dims));
    }
    // Draw the labels to the nodes
    for node in graph.nodes.values() {
        draw_label(&mut img, logic_to_pixels(node.pos, boundaries, opts.dims), node.id.as_str(), None, true);
    }

    // Done
    img
}
