use crate::point::Point;
use svg::node::element::path::Data;
use svg::node::element::{Circle, Path, SVG};
use svg::Document;

pub fn render_packing(pan_width: f64, pan_length: f64, placement: Vec<Point>) -> SVG {
    let pad = 10.0;
    let data = Data::new()
        .move_to((pad, pad))
        .line_by((0, pan_length))
        .line_by((pan_width, 0))
        .line_by((0, -1.0 * pan_length))
        .close();

    let pan = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);

    let biscuits = placement.iter().map(|p| {
        Circle::new()
            .set("cx", p.x + pad)
            .set("cy", p.y + pad)
            .set("r", 1.0)
    });

    let image = Document::new()
        .set("viewBox", (0, 0, pan_width + pad, pan_length + pad))
        .add(pan);

    biscuits.fold(image, |image, b| image.add(b))
}
