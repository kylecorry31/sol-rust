#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2D { x, y }
    }

    pub fn dot(&self, other: &Vector2D) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Vector2D) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx_eq;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Vector2D { x: 1.0, y: 0.0 }, Vector2D { x: 0.0, y: 1.0 }, 0.0)]
    #[case(Vector2D { x: 1.0, y: 1.0 }, Vector2D { x: 1.0, y: 1.0 }, 2.0)]
    #[case(Vector2D { x: -1.0, y: 1.0 }, Vector2D { x: 1.0, y: -1.0 }, -2.0)]
    fn test_dot(#[case] v1: Vector2D, #[case] v2: Vector2D, #[case] expected: f32) {
        assert_approx_eq!(v1.dot(&v2), expected);
    }

    #[rstest]
    #[case(Vector2D { x: 1.0, y: 0.0 }, Vector2D { x: 0.0, y: 1.0 }, 1.0)]
    #[case(Vector2D { x: 1.0, y: 1.0 }, Vector2D { x: 1.0, y: 1.0 }, 0.0)]
    #[case(Vector2D { x: -1.0, y: 1.0 }, Vector2D { x: 1.0, y: -1.0 }, 0.0)]
    fn test_cross(#[case] v1: Vector2D, #[case] v2: Vector2D, #[case] expected: f32) {
        assert_approx_eq!(v1.cross(&v2), expected);
    }

    #[rstest]
    #[case(Vector2D { x: 3.0, y: 4.0 }, 5.0)]
    #[case(Vector2D { x: 0.0, y: 1.0 }, 1.0)]
    #[case(Vector2D { x: 1.0, y: 0.0 }, 1.0)]
    fn test_magnitude(#[case] v: Vector2D, #[case] expected: f32) {
        assert_approx_eq!(v.magnitude(), expected);
    }
}
