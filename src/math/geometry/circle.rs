use super::{Line, Point2D, Polygon, Shape2D, Translate2D};

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

    fn contains(&self, point: &Point2D) -> bool {
        let distance = self.center.distance(point);
        distance <= self.radius
    }

    fn intersects_circle(&self, circle: &Circle) -> bool {
        let distance = self.center.distance(&circle.center);
        distance <= self.radius + circle.radius
    }

    fn intersects_line(&self, line: &Line) -> bool {
        // Calculate vector from line start to center
        let v = Point2D::new(self.center.x - line.start.x, self.center.y - line.start.y);

        // Calculate vector representing line direction
        let dir = Point2D::new(line.end.x - line.start.x, line.end.y - line.start.y);

        // Calculate squared length of line
        let len_sq = dir.x * dir.x + dir.y * dir.y;

        // Calculate dot product of v and dir
        let dot = v.x * dir.x + v.y * dir.y;

        // Calculate closest point parameter
        let t = (dot / len_sq).clamp(0.0, 1.0);

        // Calculate closest point on line
        let closest = Point2D::new(line.start.x + t * dir.x, line.start.y + t * dir.y);
        self.contains(&closest)
    }

    fn intersects_polygon(&self, polygon: &Polygon) -> bool {
        // Circle is inside the polygon
        if polygon.contains(&self.center) {
            return true;
        }

        // An edge of the polygon intersects / is contained by the circle
        for edge in polygon.edges() {
            if self.intersects_line(&edge) {
                return true;
            }
        }

        // No intersection
        false
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

    #[rstest]
    #[case(0.0, 0.0, 1.0, 0.0, 0.0, true)] // Point at center
    #[case(0.0, 0.0, 1.0, 1.0, 0.0, true)] // Point on edge
    #[case(0.0, 0.0, 2.0, 1.0, 1.0, true)] // Point inside
    #[case(0.0, 0.0, 1.0, 2.0, 2.0, false)] // Point outside
    fn test_circle_contains(
        #[case] cx: f32,
        #[case] cy: f32,
        #[case] r: f32,
        #[case] px: f32,
        #[case] py: f32,
        #[case] expected: bool,
    ) {
        let circle = Circle {
            center: Point2D::new(cx, cy),
            radius: r,
        };
        let point = Point2D::new(px, py);
        assert_eq!(circle.contains(&point), expected);
    }

    #[rstest]
    #[case(0.0, 0.0, 1.0, 2.0, 0.0, 1.0, true)] // Overlapping circles
    #[case(0.0, 0.0, 1.0, 3.0, 0.0, 1.0, false)] // Non-overlapping circles
    #[case(0.0, 0.0, 1.0, 1.0, 0.0, 1.0, true)] // Touching circles
    fn test_circle_intersects_circle(
        #[case] c1x: f32,
        #[case] c1y: f32,
        #[case] r1: f32,
        #[case] c2x: f32,
        #[case] c2y: f32,
        #[case] r2: f32,
        #[case] expected: bool,
    ) {
        let circle1 = Circle::new(Point2D::new(c1x, c1y), r1);
        let circle2 = Circle::new(Point2D::new(c2x, c2y), r2);
        assert_eq!(circle1.intersects_circle(&circle2), expected);
    }

    #[rstest]
    #[case(0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, true)] // Line through circle
    #[case(0.0, 0.0, 1.0, 2.0, 0.0, 3.0, 0.0, false)] // Line outside circle
    #[case(0.0, 0.0, 1.0, -1.0, 1.0, 1.0, 1.0, true)] // Line touching circle
    #[case(0.0, 0.0, 2.0, -0.5, 0.0, 0.5, 0.0, true)] // Line fully inside circle
    fn test_circle_intersects_line(
        #[case] cx: f32,
        #[case] cy: f32,
        #[case] r: f32,
        #[case] lx1: f32,
        #[case] ly1: f32,
        #[case] lx2: f32,
        #[case] ly2: f32,
        #[case] expected: bool,
    ) {
        let circle = Circle::new(Point2D::new(cx, cy), r);
        let line = super::Line::new(Point2D::new(lx1, ly1), Point2D::new(lx2, ly2));
        assert_eq!(circle.intersects_line(&line), expected);
    }

    #[rstest]
    #[case(0.0, 0.0, 1.0, -1.0, -1.0, 2.0, 2.0, true)] // Circle overlaps rectangle
    #[case(0.0, 0.0, 1.0, 2.0, 2.0, 1.0, 1.0, false)] // Circle outside rectangle
    #[case(0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, true)] // Circle touching rectangle
    #[case(0.0, 0.0, 0.5, -0.25, -0.25, 0.5, 0.5, true)] // Circle inside rectangle
    #[case(1.0, 1.0, 2.0, 0.5, 0.5, 1.0, 1.0, true)] // Rectangle inside circle
    fn test_circle_intersects_rectangle(
        #[case] cx: f32,
        #[case] cy: f32,
        #[case] r: f32,
        #[case] rx: f32,
        #[case] ry: f32,
        #[case] width: f32,
        #[case] height: f32,
        #[case] expected: bool,
    ) {
        let circle = Circle::new(Point2D::new(cx, cy), r);
        let rectangle = Polygon::rectangle_from_sides(width, height).translate(rx, ry);
        assert_eq!(circle.intersects_polygon(&rectangle.into()), expected);
    }
}
