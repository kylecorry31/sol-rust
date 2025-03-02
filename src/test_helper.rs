#[macro_export]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr) => {
        assert!(
            ($left - $right).abs() <= 1e-5,
            "assertion failed: `(left ≈ right)`\n  left: `{}`\n right: `{}`\n precision: `{}`",
            $left,
            $right,
            1e-5
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

#[macro_export]
macro_rules! assert_tensor_eq {
    ($left:expr, $right:expr) => {
        assert_eq!($left.len(), $right.len());
        for (row1, row2) in $left.iter().zip($right.iter()) {
            assert_eq!(row1.len(), row2.len());
            for (elem1, elem2) in row1.iter().zip(row2.iter()) {
                assert_approx_eq!(elem1, elem2);
            }
        }
    };
}
