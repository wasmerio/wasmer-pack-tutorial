// use crate::geometry::Circle;
use crate::geometry::Point;

wai_bindgen_rust::export!("geometry.wai");

struct Geometry;

impl geometry::Geometry for Geometry {
    fn distance_between(p1: Point, p2: Point) -> f32 {
        let Point { x: x1, y: y1 } = p1;
        let Point { x: x2, y: y2 } = p2;

        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }
}
