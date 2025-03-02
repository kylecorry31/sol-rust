pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

pub fn combination(n: u32, k: u32) -> u32 {
    factorial(n) / (factorial(k) * factorial(n - k))
}

pub fn permutation(n: u32, k: u32) -> u32 {
    factorial(n) / factorial(n - k)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 1)]
    #[case(1, 1)]
    #[case(2, 2)]
    #[case(3, 6)]
    #[case(4, 24)]
    #[case(5, 120)]
    #[case(6, 720)]
    #[case(7, 5040)]
    fn test_factorial(#[case] input: u32, #[case] expected: u32) {
        let result = factorial(input);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(5, 3, 10)]
    #[case(7, 3, 35)]
    #[case(6, 2, 15)]
    #[case(8, 4, 70)]
    fn test_combination(#[case] n: u32, #[case] k: u32, #[case] expected: u32) {
        let result = combination(n, k);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(5, 3, 60)]
    #[case(7, 3, 210)]
    #[case(6, 2, 30)]
    #[case(8, 4, 1680)]
    fn test_permutation(#[case] n: u32, #[case] k: u32, #[case] expected: u32) {
        let result = permutation(n, k);
        assert_eq!(expected, result);
    }
}
