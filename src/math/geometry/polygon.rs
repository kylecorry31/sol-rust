use super::{Line, Point2D, Shape2D, Translate2D};

#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    pub vertices: Vec<Point2D>,
}

impl Shape2D for Polygon {
    fn area(&self) -> f32 {
        let mut area = 0.0;
        let n = self.vertices.len();
        for i in 0..n {
            let j = (i + 1) % n;
            area += self.vertices[i].x * self.vertices[j].y;
            area -= self.vertices[j].x * self.vertices[i].y;
        }
        area.abs() / 2.0
    }

    fn perimeter(&self) -> f32 {
        let mut perimeter = 0.0;
        let n = self.vertices.len();
        for i in 0..n {
            let j = (i + 1) % n;
            perimeter += self.vertices[i].distance(&self.vertices[j]);
        }
        perimeter
    }

    fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
}

impl Polygon {
    pub fn new(vertices: Vec<Point2D>) -> Self {
        Polygon { vertices }
    }

    pub fn edges(&self) -> Vec<Line> {
        let mut edges = Vec::with_capacity(self.vertices.len());
        for i in 0..self.vertices.len() {
            let start = self.vertices[i];
            let end = self.vertices[(i + 1) % self.vertices.len()];
            edges.push(Line::new(start, end));
        }
        edges
    }

    // Helper methods for common shapes
    pub fn triangle(p1: Point2D, p2: Point2D, p3: Point2D) -> Self {
        Polygon {
            vertices: vec![p1, p2, p3],
        }
    }

    pub fn rectangle(p1: Point2D, p2: Point2D, p3: Point2D, p4: Point2D) -> Self {
        Polygon {
            vertices: vec![p1, p2, p3, p4],
        }
    }

    /// A rectangle with the bottom left corner at the origin
    pub fn rectangle_from_sides(width: f32, height: f32) -> Self {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(width, 0.0);
        let p3 = Point2D::new(width, height);
        let p4 = Point2D::new(0.0, height);
        Polygon {
            vertices: vec![p1, p2, p3, p4],
        }
    }
}

impl Translate2D for Polygon {
    fn translate(&self, dx: f32, dy: f32) -> Self {
        Polygon {
            vertices: self.vertices.iter().map(|v| v.translate(dx, dy)).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.5)]
    fn test_triangle_area(
        #[case] x1: f32,
        #[case] y1: f32,
        #[case] x2: f32,
        #[case] y2: f32,
        #[case] x3: f32,
        #[case] y3: f32,
        #[case] expected: f32,
    ) {
        let p1 = Point2D::new(x1, y1);
        let p2 = Point2D::new(x2, y2);
        let p3 = Point2D::new(x3, y3);
        let triangle = Polygon::triangle(p1, p2, p3);
        assert_eq!(triangle.area(), expected);
    }

    #[rstest]
    #[case(0.0, 0.0, 2.0, 0.0, 2.0, 1.0, 0.0, 1.0, 6.0)]
    fn test_rectangle_perimeter(
        #[case] x1: f32,
        #[case] y1: f32,
        #[case] x2: f32,
        #[case] y2: f32,
        #[case] x3: f32,
        #[case] y3: f32,
        #[case] x4: f32,
        #[case] y4: f32,
        #[case] expected: f32,
    ) {
        let p1 = Point2D::new(x1, y1);
        let p2 = Point2D::new(x2, y2);
        let p3 = Point2D::new(x3, y3);
        let p4 = Point2D::new(x4, y4);
        let rectangle = Polygon::rectangle(p1, p2, p3, p4);
        assert!((rectangle.perimeter() - expected).abs() < f32::EPSILON);
    }

    #[rstest]
    #[case(2.0, 3.0, 6.0)]
    fn test_rectangle_from_sides_area(
        #[case] width: f32,
        #[case] height: f32,
        #[case] expected: f32,
    ) {
        let rectangle = Polygon::rectangle_from_sides(width, height);
        assert_eq!(rectangle.area(), expected);
    }

    #[rstest]
    #[case(1.0, 1.0, 2.0, 2.0)]
    fn test_polygon_translate(
        #[case] dx: f32,
        #[case] dy: f32,
        #[case] new_x: f32,
        #[case] new_y: f32,
    ) {
        let polygon = Polygon::rectangle_from_sides(1.0, 1.0);
        let translated = polygon.translate(dx, dy);
        assert_eq!(translated.vertices[2], Point2D::new(new_x, new_y));
    }
}
