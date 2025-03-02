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
}
