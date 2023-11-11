use crate::point::Point;
use svg::node::element::{Circle, Rectangle, SVG};
use svg::Document;

pub fn render_packing(pan_width: f64, pan_length: f64, placement: Vec<Point>) -> SVG {
    let pad = 0.1 * pan_width;
    let biscuit_radius = 0.02 * pan_width;

    let pan = Rectangle::new()
        // .set("x", pad)
        // .set("y", pad)
        .set("width", pan_width)
        .set("height", pan_length)
        .set("fill", "lightgrey")
        .set("stroke", "black")
        .set("stroke-width", 1);

    let biscuits = placement.iter().map(|p| {
        Circle::new()
            .set("cx", p.x)
            .set("cy", p.y)
            .set("r", biscuit_radius)
            .set("fill", "darkgreen")
    });

    let image = Document::new()
        // .set("height", pan_length)
        // .set("width", pan_width)
        .set("viewBox", (0, 0, pan_length, pan_width))
        .add(pan);

    biscuits.fold(image, |image, b| image.add(b))
}
