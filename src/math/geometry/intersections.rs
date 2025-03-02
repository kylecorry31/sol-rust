use crate::math::{
    algebra::solve_quadratic, arithmetic::square, linear_algebra::Tensor,
    utils::is_approximately_zero,
};

use super::{Circle, Line, Point2D, Polygon};

pub fn contains_line_point(line: &Line, point: &Point2D) -> bool {
    let (x1, y1) = (line.start.x, line.start.y);
    let (x2, y2) = (line.end.x, line.end.y);
    let (px, py) = (point.x, point.y);

    let dx = x2 - x1;
    let dy = y2 - y1;

    // Calculate distance from point to line
    let dist = ((py - y1) * dx - (px - x1) * dy).abs() / (dx * dx + dy * dy).sqrt();

    // Calculate parameter t to find closest point on line
    let t = ((px - x1) * dx + (py - y1) * dy) / (dx * dx + dy * dy);

    is_approximately_zero(dist) && (0.0..=1.0).contains(&t)
}

pub fn contains_circle_point(circle: &Circle, point: &Point2D) -> bool {
    circle.center.distance(point) <= circle.radius
}

pub fn contains_polygon_point(polygon: &Polygon, point: &Point2D) -> bool {
    let mut inside = false;
    let n = polygon.vertices.len();
    for i in 0..n {
        let j = (i + 1) % n;
        let p1 = polygon.vertices[i];
        let p2 = polygon.vertices[j];

        // Check if point is on edge
        let d = (point.x - p1.x) * (p2.y - p1.y) - (point.y - p1.y) * (p2.x - p1.x);
        if is_approximately_zero(d) {
            let t = ((point.x - p1.x) * (p2.x - p1.x) + (point.y - p1.y) * (p2.y - p1.y))
                / p1.distance(&p2);
            if (0.0..=1.0).contains(&t) {
                return true;
            }
        }

        if (p1.y > point.y) != (p2.y > point.y)
            && point.x < (p2.x - p1.x) * (point.y - p1.y) / (p2.y - p1.y) + p1.x
        {
            inside = !inside;
        }
    }
    inside
}

pub fn intersects_line_line(line1: &Line, line2: &Line) -> bool {
    let (p1, q1) = (line1.start, line1.end);
    let (p2, q2) = (line2.start, line2.end);

    // Helper function to find orientation of ordered triplet (p, q, r): https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
    let orientation = |p: &Point2D, q: &Point2D, r: &Point2D| -> i32 {
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        if is_approximately_zero(val) {
            0
        } else if val > 0.0 {
            1
        } else {
            2
        }
    };

    let o1 = orientation(&p1, &q1, &p2);
    let o2 = orientation(&p1, &q1, &q2);
    let o3 = orientation(&p2, &q2, &p1);
    let o4 = orientation(&p2, &q2, &q1);

    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Special Cases
    if o1 == 0 && contains_line_point(&Line::new(p1, q1), &p2) {
        return true;
    }

    if o2 == 0 && contains_line_point(&Line::new(p1, q1), &q2) {
        return true;
    }

    if o3 == 0 && contains_line_point(&Line::new(p2, q2), &p1) {
        return true;
    }

    if o4 == 0 && contains_line_point(&Line::new(p2, q2), &q1) {
        return true;
    }

    false
}

pub fn intercepts_circle_line(circle: &Circle, line: &Line) -> Vec<Point2D> {
    let center_direction = circle.center - line.start;
    let line_direction = line.end - line.start;

    // Calculate quadratic coefficients
    let a = line_direction.dot(&line_direction);
    let b = 2.0 * center_direction.dot(&line_direction);
    let c = center_direction.dot(&center_direction) - square(circle.radius);

    let roots = solve_quadratic(a, b, c);
    let mut intersections = Vec::new();
    for t in roots {
        if (0.0..=1.0).contains(&t) {
            intersections.push(Point2D::new(
                line.start.x + t * line_direction.x,
                line.start.y + t * line_direction.y,
            ));
        }
    }
    intersections
}

pub fn intersects_circle_line(circle: &Circle, line: &Line) -> bool {
    contains_circle_point(circle, &line.start)
        || contains_circle_point(circle, &line.end)
        || !intercepts_circle_line(circle, line).is_empty()
}

pub fn intersects_circle_circle(circle1: &Circle, circle2: &Circle) -> bool {
    circle1.center.distance(&circle2.center) <= circle1.radius + circle2.radius
}

pub fn intersects_circle_polygon(circle: &Circle, polygon: &Polygon) -> bool {
    // Circle is inside the polygon
    if contains_polygon_point(polygon, &circle.center) {
        return true;
    }

    // An edge of the polygon intersects / is contained by the circle
    for edge in polygon.edges() {
        if intersects_circle_line(circle, &edge) {
            return true;
        }
    }

    // No intersection
    false
}

pub fn intersects_polygon_line(polygon: &Polygon, line: &Line) -> bool {
    // Check to see if each line intersects with an edge OR if the line's start point is contained by the polygon
    for edge in polygon.edges() {
        if intersects_line_line(&edge, line) {
            return true;
        }
    }
    contains_polygon_point(polygon, &line.start)
}

pub fn intersects_polygon_polygon(polygon1: &Polygon, polygon2: &Polygon) -> bool {
    for edge in polygon1.edges() {
        if intersects_polygon_line(polygon2, &edge) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case((0.0, 0.0), (1.0, 0.0), 0.5, 0.0, true)] // Point on horizontal line
    #[case((0.0, 0.0), (1.0, 0.0), 1.0, 0.0, true)] // Point at end of horizontal line
    #[case((0.0, 0.0), (1.0, 0.0), 0.5, 0.5, false)] // Point above horizontal line
    #[case((0.0, 0.0), (1.0, 0.0), -0.5, 0.0, false)] // Point before horizontal line start
    #[case((0.0, 0.0), (0.0, 1.0), 0.0, 0.5, true)] // Point on vertical line
    #[case((0.0, 0.0), (0.0, 1.0), 0.5, 0.5, false)] // Point beside vertical line
    #[case((0.0, 0.0), (1.0, 1.0), 0.5, 0.5, true)] // Point on diagonal line
    #[case((0.0, 0.0), (1.0, 1.0), 0.5, 0.0, false)] // Point below diagonal line
    fn test_contains_line_point(
        #[case] start: (f32, f32),
        #[case] end: (f32, f32),
        #[case] px: f32,
        #[case] py: f32,
        #[case] expected: bool,
    ) {
        let line = Line::new(Point2D::new(start.0, start.1), Point2D::new(end.0, end.1));
        let point = Point2D::new(px, py);
        assert_eq!(contains_line_point(&line, &point), expected);
    }

    #[rstest]
    #[case(0.0, 0.0, true)] // Point at center
    #[case(1.0, 0.0, true)] // Point on edge
    #[case(0.707, 0.707, true)] // Point on edge at 45 degrees
    #[case(2.0, 0.0, false)] // Point outside circle
    #[case(1.5, 1.5, false)] // Point outside circle diagonally
    fn test_contains_circle_point(#[case] x: f32, #[case] y: f32, #[case] expected: bool) {
        let circle = Circle::new(Point2D::new(0.0, 0.0), 1.0);
        let point = Point2D::new(x, y);
        assert_eq!(contains_circle_point(&circle, &point), expected);
    }

    #[rstest]
    // Triangle tests
    #[case(vec![(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)], 0.5, 0.5, true)] // Point inside triangle
    #[case(vec![(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)], 0.5, 0.0, true)] // Point on edge
    #[case(vec![(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)], 0.0, 0.0, true)] // Point on vertex
    #[case(vec![(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)], -0.5, -0.5, false)] // Point outside
    // Rectangle tests
    #[case(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 0.5, 0.5, true)] // Point inside rectangle
    #[case(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 0.5, 0.0, true)] // Point on edge
    #[case(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 0.0, 0.0, true)] // Point on vertex
    #[case(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], -0.5, -0.5, false)] // Point outside
    fn test_contains_polygon_point(
        #[case] vertices: Vec<(f32, f32)>,
        #[case] x: f32,
        #[case] y: f32,
        #[case] expected: bool,
    ) {
        let vertices = vertices
            .into_iter()
            .map(|(x, y)| Point2D::new(x, y))
            .collect();
        let polygon = Polygon::new(vertices);
        let point = Point2D::new(x, y);
        assert_eq!(contains_polygon_point(&polygon, &point), expected);
    }

    #[rstest]
    #[case((0.0, 0.0), (1.0, 0.0), (0.5, 0.0), (1.5, 0.0), true)] // Lines overlap horizontally
    #[case((0.0, 0.0), (0.0, 1.0), (0.0, 0.5), (0.0, 1.5), true)] // Lines overlap vertically
    #[case((0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 1.0), true)] // Lines cross in middle
    #[case((0.0, 0.0), (1.0, 1.0), (0.5, 0.5), (1.5, 1.5), true)] // Lines overlap diagonally
    #[case((0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0), false)] // Parallel horizontal lines
    #[case((0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0), false)] // Parallel vertical lines
    #[case((0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (3.0, 0.0), false)] // Non-intersecting horizontal lines
    #[case((0.0, 0.0), (0.0, 1.0), (0.0, 2.0), (0.0, 3.0), false)] // Non-intersecting vertical lines
    #[case((0.0, 0.0), (1.0, 1.0), (0.0, 2.0), (1.0, 3.0), false)] // Parallel diagonal lines
    #[case((0.0, 0.0), (1.0, 0.0), (1.0, 0.0), (2.0, 0.0), true)] // Lines touch at endpoint
    fn test_intersects_line_line(
        #[case] start1: (f32, f32),
        #[case] end1: (f32, f32),
        #[case] start2: (f32, f32),
        #[case] end2: (f32, f32),
        #[case] expected: bool,
    ) {
        let line1 = Line::new(
            Point2D::new(start1.0, start1.1),
            Point2D::new(end1.0, end1.1),
        );
        let line2 = Line::new(
            Point2D::new(start2.0, start2.1),
            Point2D::new(end2.0, end2.1),
        );
        assert_eq!(intersects_line_line(&line1, &line2), expected);
    }

    #[rstest]
    #[case(0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.5, true)] // Line intersects circle
    #[case(0.0, 0.0, 1.0, 2.0, 2.0, 3.0, 0.0, false)] // Line outside circle
    #[case(0.0, 0.0, 1.0, 0.1, 0.1, 0.2, 0.5, true)] // Line inside circle
    fn test_intersects_circle_line(
        #[case] cx: f32,
        #[case] cy: f32,
        #[case] r: f32,
        #[case] x1: f32,
        #[case] y1: f32,
        #[case] x2: f32,
        #[case] y2: f32,
        #[case] expected: bool,
    ) {
        let circle = Circle::new(Point2D::new(cx, cy), r);
        let line = Line::new(Point2D::new(x1, y1), Point2D::new(x2, y2));
        assert_eq!(intersects_circle_line(&circle, &line), expected);
    }

    #[rstest]
    #[case(0.0, 0.0, 1.0, 2.0, 0.0, 1.0, true)] // Circles touching
    #[case(0.0, 0.0, 1.0, 0.5, 0.0, 1.0, true)] // Circles overlapping
    #[case(0.0, 0.0, 1.0, 3.0, 0.0, 1.0, false)] // Circles separate
    #[case(0.0, 0.0, 2.0, 0.0, 0.0, 1.0, true)] // One circle inside other
    fn test_intersects_circle_circle(
        #[case] x1: f32,
        #[case] y1: f32,
        #[case] r1: f32,
        #[case] x2: f32,
        #[case] y2: f32,
        #[case] r2: f32,
        #[case] expected: bool,
    ) {
        let circle1 = Circle::new(Point2D::new(x1, y1), r1);
        let circle2 = Circle::new(Point2D::new(x2, y2), r2);
        assert_eq!(intersects_circle_circle(&circle1, &circle2), expected);
    }

    #[rstest]
    // Triangle tests
    #[case(0.0, 0.0, 1.0, vec![(0.5, 0.5), (1.5, 0.5), (1.0, 1.5)], true)] // Circle intersects triangle
    #[case(0.0, 0.0, 0.1, vec![(2.0, 0.0), (3.0, 0.0), (2.5, 1.0)], false)] // Circle outside triangle
    #[case(1.0, 1.0, 0.1, vec![(0.0, 0.0), (2.0, 0.0), (1.0, 2.0)], true)] // Circle inside triangle
    // Rectangle tests
    #[case(0.0, 0.0, 1.0, vec![(0.5, 0.5), (1.5, 0.5), (1.5, 1.5), (0.5, 1.5)], true)] // Circle intersects rectangle
    #[case(0.0, 0.0, 0.1, vec![(2.0, 2.0), (3.0, 2.0), (3.0, 3.0), (2.0, 3.0)], false)] // Circle outside rectangle
    #[case(1.0, 1.0, 0.1, vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)], true)] // Circle inside rectangle
    fn test_intersects_circle_polygon(
        #[case] cx: f32,
        #[case] cy: f32,
        #[case] r: f32,
        #[case] vertices: Vec<(f32, f32)>,
        #[case] expected: bool,
    ) {
        let circle = Circle::new(Point2D::new(cx, cy), r);
        let vertices = vertices
            .into_iter()
            .map(|(x, y)| Point2D::new(x, y))
            .collect();
        let polygon = Polygon::new(vertices);
        assert_eq!(intersects_circle_polygon(&circle, &polygon), expected);
    }

    #[rstest]
    // Triangle tests
    #[case(vec![(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)], 0.25, 0.25, 0.75, 0.75, true)] // Line intersects triangle
    #[case(vec![(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)], -1.0, -1.0, -0.5, -0.5, false)] // Line outside triangle
    // Rectangle tests
    #[case(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 0.25, 0.25, 0.75, 0.75, true)] // Line intersects rectangle
    #[case(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], -1.0, -1.0, -0.5, -0.5, false)] // Line outside rectangle
    fn test_intersects_polygon_line(
        #[case] vertices: Vec<(f32, f32)>,
        #[case] x1: f32,
        #[case] y1: f32,
        #[case] x2: f32,
        #[case] y2: f32,
        #[case] expected: bool,
    ) {
        let vertices = vertices
            .into_iter()
            .map(|(x, y)| Point2D::new(x, y))
            .collect();
        let polygon = Polygon::new(vertices);
        let line = Line::new(Point2D::new(x1, y1), Point2D::new(x2, y2));
        assert_eq!(intersects_polygon_line(&polygon, &line), expected);
    }

    #[rstest]
    // Triangle-Triangle tests
    #[case(
        vec![(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)],
        vec![(0.25, 0.25), (1.25, 0.25), (0.75, 1.25)],
        true
    )] // Triangles intersect
    #[case(
        vec![(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)],
        vec![(2.0, 2.0), (3.0, 2.0), (2.5, 3.0)],
        false
    )] // Triangles separate
    // Rectangle-Rectangle tests
    #[case(
        vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        vec![(0.5, 0.5), (1.5, 0.5), (1.5, 1.5), (0.5, 1.5)],
        true
    )] // Rectangles intersect
    #[case(
        vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        vec![(2.0, 2.0), (3.0, 2.0), (3.0, 3.0), (2.0, 3.0)],
        false
    )] // Rectangles separate
    fn test_intersects_polygon_polygon(
        #[case] vertices1: Vec<(f32, f32)>,
        #[case] vertices2: Vec<(f32, f32)>,
        #[case] expected: bool,
    ) {
        let vertices1 = vertices1
            .into_iter()
            .map(|(x, y)| Point2D::new(x, y))
            .collect();
        let vertices2 = vertices2
            .into_iter()
            .map(|(x, y)| Point2D::new(x, y))
            .collect();
        let polygon1 = Polygon::new(vertices1);
        let polygon2 = Polygon::new(vertices2);
        assert_eq!(intersects_polygon_polygon(&polygon1, &polygon2), expected);
    }
}
