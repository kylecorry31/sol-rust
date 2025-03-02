#[macro_export]
macro_rules! assert_approx_eq {
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
