use crate::point::Point;

#[test]
fn test_distance() {
    let origin = Point::new(0.0, 0.0);
    assert_eq!(1.0, origin.distance(&Point::new(0.0, 1.0)));
    assert_eq!(2_f64.sqrt(), origin.distance(&Point::new(1.0, 1.0)));
    assert_eq!(
        origin.distance(&Point::new(0.0, -1.0)),
        origin.distance(&Point::new(0.0, 1.0))
    );
}

#[test]
fn test_constructor() {
    let p = Point { x: 1.0, y: 2.0 };
    assert_eq!(p, Point::new(1.0, 2.0))
}
