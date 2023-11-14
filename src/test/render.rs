use crate::point::Point;
use crate::render;

#[test]
fn rendered_shape_count() {
    let arrangement = vec![
        Point::new(25.0, 25.0),
        Point::new(50.0, 50.0),
        Point::new(75.0, 75.0),
    ];
    let svg = render::render_packing(100.0, 100.0, &arrangement);

    // output should only have the biscuits and pan shapes
    assert_eq!(svg.get_children().len(), arrangement.len() + 1)
}
