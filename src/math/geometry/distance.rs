use super::Point2D;

pub fn euclidean_distance(p1: &Point2D, p2: &Point2D) -> f32 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

pub fn manhattan_distance(p1: &Point2D, p2: &Point2D) -> f32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

pub fn chebyshev_distance(p1: &Point2D, p2: &Point2D) -> f32 {
    (p1.x - p2.x).abs().max((p1.y - p2.y).abs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 3.0, y: 4.0 }, 5.0)]
    #[case(Point2D { x: 1.0, y: 1.0 }, Point2D { x: 4.0, y: 5.0 }, 5.0)]
    fn test_euclidean_distance(#[case] p1: Point2D, #[case] p2: Point2D, #[case] expected: f32) {
        assert_eq!(euclidean_distance(&p1, &p2), expected);
    }

    #[rstest]
    #[case(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 3.0, y: 4.0 }, 7.0)]
    #[case(Point2D { x: 1.0, y: 1.0 }, Point2D { x: 4.0, y: 5.0 }, 7.0)]
    fn test_manhattan_distance(#[case] p1: Point2D, #[case] p2: Point2D, #[case] expected: f32) {
        assert_eq!(manhattan_distance(&p1, &p2), expected);
    }

    #[rstest]
    #[case(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 3.0, y: 4.0 }, 4.0)]
    #[case(Point2D { x: 1.0, y: 1.0 }, Point2D { x: 4.0, y: 5.0 }, 4.0)]
    fn test_chebyshev_distance(#[case] p1: Point2D, #[case] p2: Point2D, #[case] expected: f32) {
        assert_eq!(chebyshev_distance(&p1, &p2), expected);
    }
}
