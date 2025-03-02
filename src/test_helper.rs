#[macro_export]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr) => {
        assert!(
            ($left - $right).abs() <= f64::EPSILON,
            "assertion failed: `(left ≈ right)`\n  left: `{}`\n right: `{}`\n precision: `{}`",
            $left,
            $right,
            f64::EPSILON
        );
    };
    ($left:expr, $right:expr, $precision:expr) => {
        assert!(
            ($left - $right).abs() <= $precision,
            "assertion failed: `(left ≈ right)`\n  left: `{}`\n right: `{}`\n precision: `{}`",
            $left,
            $right,
            $precision
        );
    };
}
