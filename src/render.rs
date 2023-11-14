use crate::point::Point;
use svg::node::element::{Circle, Rectangle, SVG};
use svg::Document;

pub fn render_packing(pan_width: f64, pan_length: f64, placement: &[Point]) -> SVG {
    let view_box_width = 1000.0;
    let length_ratio = pan_length / pan_width;
    let biscuit_radius = 0.02 * pan_width;

    let pan = Rectangle::new()
        .set("width", pan_width)
        .set("height", pan_length)
        .set("fill", "lightgrey")
        .set("stroke", "black")
        .set("stroke-width", "2%");

    let biscuits = placement.iter().map(|p| {
        Circle::new()
            .set("cx", p.x)
            .set("cy", p.y)
            .set("r", biscuit_radius)
            .set("fill", "darkgreen")
    });

    let image = Document::new()
        .set("height", view_box_width * length_ratio)
        .set("width", view_box_width)
        .set("viewBox", (0, 0, pan_width, pan_length))
        .add(pan);

    biscuits.fold(image, |image, b| image.add(b))
}
