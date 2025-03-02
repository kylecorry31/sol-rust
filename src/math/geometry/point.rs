use crate::math::linear_algebra::Vector;

use super::{Translate2D, euclidean_distance};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn distance(&self, other: &Point2D) -> f32 {
        euclidean_distance(self, other)
    }

    pub fn angle(&self, other: &Point2D) -> f32 {
        (other.y - self.y).atan2(other.x - self.x)
    }

    pub fn new(x: f32, y: f32) -> Self {
        Point2D { x, y }
    }
}

impl std::ops::Sub for Point2D {
    type Output = Vector;

    fn sub(self, other: Self) -> Self::Output {
        Vector::new_2d(self.x - other.x, self.y - other.y)
    }
}

impl Translate2D for Point2D {
    fn translate(&self, dx: f32, dy: f32) -> Self {
        Point2D {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx_eq;

    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0.0, 0.0, 0.0, 0.0, 0.0)]
    #[case(1.0, 0.0, 0.0, 0.0, 1.0)]
    #[case(0.0, 1.0, 0.0, 0.0, 1.0)]
    #[case(3.0, 4.0, 0.0, 0.0, 5.0)]
    #[case(1.0, 1.0, 2.0, 2.0, 1.4142135)]
    fn test_distance(
        #[case] x1: f32,
        #[case] y1: f32,
        #[case] x2: f32,
        #[case] y2: f32,
        #[case] expected: f32,
    ) {
        let p1 = Point2D::new(x1, y1);
        let p2 = Point2D::new(x2, y2);
        assert!((p1.distance(&p2) - expected).abs() < 1e-6);
    }

    #[rstest]
    #[case(1.0, 2.0, 3.0, 4.0, 4.0, 6.0)]
    #[case(0.0, 0.0, 1.0, 1.0, 1.0, 1.0)]
    #[case(-1.0, -2.0, 2.0, 3.0, 1.0, 1.0)]
    fn test_translate(
        #[case] x: f32,
        #[case] y: f32,
        #[case] dx: f32,
        #[case] dy: f32,
        #[case] expected_x: f32,
        #[case] expected_y: f32,
    ) {
        let point = Point2D::new(x, y);
        let translated = point.translate(dx, dy);
        assert_eq!(translated.x, expected_x);
        assert_eq!(translated.y, expected_y);
    }

    #[rstest]
    #[case(1.0, 0.0, 1.0, 1.0, std::f32::consts::PI / 2.0)]
    #[case(1.0, 0.0, 0.0, 1.0, 3.0 * std::f32::consts::PI / 4.0)]
    #[case(0.0, 0.0, -1.0, 0.0, std::f32::consts::PI)]
    #[case(0.0, 0.0, 1.0, 0.0, 0.0)]
    #[case(0.0, 0.0, 0.0, -1.0, -std::f32::consts::PI / 2.0)]
    fn test_angle(
        #[case] x1: f32,
        #[case] y1: f32,
        #[case] x2: f32,
        #[case] y2: f32,
        #[case] expected: f32,
    ) {
        let p1 = Point2D::new(x1, y1);
        let p2 = Point2D::new(x2, y2);
        assert_approx_eq!(p1.angle(&p2), expected);
    }

    #[rstest]
    #[case(1.0, 2.0, 3.0, 4.0, -2.0, -2.0)]
    #[case(0.0, 0.0, 1.0, 1.0, -1.0, -1.0)]
    #[case(-1.0, -2.0, 2.0, 3.0, -3.0, -5.0)]
    fn test_sub(
        #[case] x1: f32,
        #[case] y1: f32,
        #[case] x2: f32,
        #[case] y2: f32,
        #[case] expected_x: f32,
        #[case] expected_y: f32,
    ) {
        let p1 = Point2D::new(x1, y1);
        let p2 = Point2D::new(x2, y2);
        let result = p1 - p2;
        assert_eq!(result.x, expected_x);
        assert_eq!(result.y, expected_y);
    }
}
