use crate::math::utils::is_approximately_zero;

use super::{Point2D, Translate2D};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub start: Point2D,
    pub end: Point2D,
}

impl Line {
    pub fn len(&self) -> f32 {
        self.start.distance(&self.end)
    }

    pub fn new(start: Point2D, end: Point2D) -> Self {
        Line { start, end }
    }

    pub fn contains(&self, point: &Point2D) -> bool {
        let (x1, y1) = (self.start.x, self.start.y);
        let (x2, y2) = (self.end.x, self.end.y);
        let (px, py) = (point.x, point.y);

        let dx = x2 - x1;
        let dy = y2 - y1;

        let t = ((px - x1) * dx + (py - y1) * dy) / (dx * dx + dy * dy);

        // Calculate distance from point to line
        let dist = ((py - y1) * dx - (px - x1) * dy).abs() / (dx * dx + dy * dy).sqrt();

        is_approximately_zero(dist) && (0.0..=1.0).contains(&t)
    }

    pub fn intersects_line(&self, other: &Self) -> bool {
        let (a, b) = (self.start, self.end);
        let (c, d) = (other.start, other.end);

        let denominator = (b.x - a.x) * (d.y - c.y) - (b.y - a.y) * (d.x - c.x);
        if denominator == 0.0 {
            return false;
        }

        let t = ((c.x - a.x) * (d.y - c.y) - (c.y - a.y) * (d.x - c.x)) / denominator;
        let u = ((c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)) / denominator;

        (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u)
    }
}

impl Translate2D for Line {
    fn translate(&self, dx: f32, dy: f32) -> Self {
        Line {
            start: self.start.translate(dx, dy),
            end: self.end.translate(dx, dy),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Point2D::new(0.0, 0.0), Point2D::new(3.0, 4.0), 5.0)]
    #[case(Point2D::new(-1.0, -1.0), Point2D::new(2.0, 3.0), 5.0)]
    #[case(Point2D::new(1.0, 1.0), Point2D::new(1.0, 1.0), 0.0)]
    fn test_line_length(#[case] start: Point2D, #[case] end: Point2D, #[case] expected: f32) {
        let line = Line { start, end };
        assert_eq!(line.len(), expected);
    }

    #[rstest]
    #[case(
        Point2D::new(1.0, 1.0),
        Point2D::new(2.0, 2.0),
        1.0,
        1.0,
        Point2D::new(2.0, 2.0),
        Point2D::new(3.0, 3.0)
    )]
    fn test_line_translate(
        #[case] start: Point2D,
        #[case] end: Point2D,
        #[case] dx: f32,
        #[case] dy: f32,
        #[case] expected_start: Point2D,
        #[case] expected_end: Point2D,
    ) {
        let line = Line { start, end };
        let translated = line.translate(dx, dy);
        assert_eq!(translated.start, expected_start);
        assert_eq!(translated.end, expected_end);
    }

    #[rstest]
    // Point on diagonal line
    #[case(
        Point2D::new(1.0, 1.0),
        Point2D::new(3.0, 3.0),
        Point2D::new(2.0, 2.0),
        true
    )]
    // Point beyond diagonal line
    #[case(
        Point2D::new(1.0, 1.0),
        Point2D::new(3.0, 3.0),
        Point2D::new(4.0, 4.0),
        false
    )]
    // Point offset from diagonal line
    #[case(
        Point2D::new(1.0, 1.0),
        Point2D::new(3.0, 3.0),
        Point2D::new(2.0, 1.0),
        false
    )]
    // Point on vertical line
    #[case(
        Point2D::new(1.0, 1.0),
        Point2D::new(1.0, 3.0),
        Point2D::new(1.0, 2.0),
        true
    )]
    fn test_line_contains(
        #[case] start: Point2D,
        #[case] end: Point2D,
        #[case] point: Point2D,
        #[case] expected: bool,
    ) {
        let line = Line { start, end };
        assert_eq!(line.contains(&point), expected);
    }

    #[rstest]
    // Intersecting lines
    #[case(
        Point2D::new(0.0, 0.0),
        Point2D::new(2.0, 2.0),
        Point2D::new(0.0, 2.0),
        Point2D::new(2.0, 0.0),
        true
    )]
    // Parallel lines
    #[case(
        Point2D::new(0.0, 0.0),
        Point2D::new(2.0, 2.0),
        Point2D::new(1.0, 0.0),
        Point2D::new(3.0, 2.0),
        false
    )]
    // Touching endpoints
    #[case(
        Point2D::new(0.0, 0.0),
        Point2D::new(2.0, 2.0),
        Point2D::new(2.0, 2.0),
        Point2D::new(4.0, 0.0),
        true
    )]
    // Non-intersecting lines
    #[case(
        Point2D::new(0.0, 0.0),
        Point2D::new(1.0, 1.0),
        Point2D::new(3.0, 3.0),
        Point2D::new(4.0, 4.0),
        false
    )]
    fn test_line_intersects(
        #[case] start1: Point2D,
        #[case] end1: Point2D,
        #[case] start2: Point2D,
        #[case] end2: Point2D,
        #[case] expected: bool,
    ) {
        let line1 = Line {
            start: start1,
            end: end1,
        };
        let line2 = Line {
            start: start2,
            end: end2,
        };
        assert_eq!(line1.intersects_line(&line2), expected);
    }
}
