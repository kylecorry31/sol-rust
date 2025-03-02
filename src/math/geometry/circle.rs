use super::{Point2D, Shape2D, Translate2D};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub center: Point2D,
    pub radius: f32,
}

impl Shape2D for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius.powi(2)
    }

    fn perimeter(&self) -> f32 {
        2.0 * std::f32::consts::PI * self.radius
    }

    fn vertex_count(&self) -> usize {
        0
    }
}

impl Translate2D for Circle {
    fn translate(&self, dx: f32, dy: f32) -> Self {
        Circle {
            center: self.center.translate(dx, dy),
            radius: self.radius,
        }
    }
}

impl Circle {
    pub fn new(center: Point2D, radius: f32) -> Self {
        Circle { center, radius }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0.0, 0.0, 1.0, std::f32::consts::PI)] // Center at origin, radius 1
    #[case(1.0, 1.0, 2.0, 4.0 * std::f32::consts::PI)] // Center at (1,1), radius 2
    fn test_circle_area(#[case] x: f32, #[case] y: f32, #[case] r: f32, #[case] expected: f32) {
        let circle = Circle {
            center: Point2D::new(x, y),
            radius: r,
        };
        assert_eq!(circle.area(), expected);
    }

    #[rstest]
    #[case(0.0, 0.0, 1.0, 2.0 * std::f32::consts::PI)] // Center at origin, radius 1
    #[case(1.0, 1.0, 2.0, 4.0 * std::f32::consts::PI)] // Center at (1,1), radius 2
    fn test_circle_perimeter(
        #[case] x: f32,
        #[case] y: f32,
        #[case] r: f32,
        #[case] expected: f32,
    ) {
        let circle = Circle {
            center: Point2D::new(x, y),
            radius: r,
        };
        assert_eq!(circle.perimeter(), expected);
    }

    #[rstest]
    #[case(0.0, 0.0, 1.0, 1.0, 1.0)] // Translate by (1,1)
    #[case(1.0, 1.0, 2.0, -1.0, -1.0)] // Translate by (-1,-1)
    fn test_circle_translate(
        #[case] x: f32,
        #[case] y: f32,
        #[case] r: f32,
        #[case] dx: f32,
        #[case] dy: f32,
    ) {
        let circle = Circle {
            center: Point2D::new(x, y),
            radius: r,
        };
        let translated = circle.translate(dx, dy);
        assert_eq!(translated.center, Point2D::new(x + dx, y + dy));
        assert_eq!(translated.radius, r);
    }

    #[rstest]
    fn test_circle_vertex_count() {
        let circle = Circle {
            center: Point2D::new(0.0, 0.0),
            radius: 1.0,
        };
        assert_eq!(circle.vertex_count(), 0);
    }
}
