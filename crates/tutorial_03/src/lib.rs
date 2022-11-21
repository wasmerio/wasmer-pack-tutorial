use crate::geometry::{Circle, MultiLine, Point};

wai_bindgen_rust::export!("geometry.wai");

struct Geometry;

impl geometry::Geometry for Geometry {
    fn distance_between(p1: Point, p2: Point) -> f32 {
        let Point { x: x1, y: y1 } = p1;
        let Point { x: x2, y: y2 } = p2;

        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }
    fn perimeter_of_circle(c: Circle) -> f32 {
        let Circle { center: _, radius } = c;
        (2.0 * 22.0 * radius as f32) / 7.0
    }
    fn area_of_circle(c: Circle) -> f32 {
        let Circle { center: _, radius } = c;
        (22.0 * (radius * radius) as f32) / 7.0
    }
    fn multi_line_length(l: MultiLine) -> f32 {
        if l.points.len() == 0 {
            return 0.0;
        }
        let mut result = 0.0;
        for i in 1..l.points.len() {
            let p1 = l.points[i - 1];
            let p2 = l.points[i];
            result += Geometry::distance_between(p1, p2);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Geometry as _;

    #[test]
    fn distance_between_two_points() {
        let p1 = Point { x: 2.0, y: 5.0 };
        let p2 = Point { x: 6.0, y: 8.0 };
        let distance = Geometry::distance_between(p1, p2);
        assert_eq!(distance, 5.0);
    }
    #[test]
    fn check_perimeter() {
        let c = Circle {
            center: Point { x: 5.0, y: 3.0 },
            radius: 7.0,
        };
        let perimeter = Geometry::perimeter_of_circle(c);
        assert_eq!(perimeter, 44.0);
    }
    #[test]
    fn check_area() {
        let c = Circle {
            center: Point { x: 5.0, y: 3.0 },
            radius: 3.5,
        };
        let area = Geometry::area_of_circle(c);
        assert_eq!(area, 38.5);
    }
    #[test]
    fn check_multiline_length() {
        let multiline = MultiLine {
            points: vec![
                Point { x: 1.0, y: 0.0 },
                Point { x: 4.0, y: 5.0 },
                Point { x: 4.0, y: -5.0 },
            ],
        };
        let length = Geometry::multi_line_length(multiline);
        assert_eq!(length, 15.830952);
    }
}
