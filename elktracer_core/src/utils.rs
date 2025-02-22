#[cfg(test)]
#[macro_use]
mod test_utils {
    /// Asserts that two floating-point numbers are approximately equal within a given epsilon.
    #[macro_export]
    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr, $epsilon:expr) => {
            if ($a - $b).abs() >= $epsilon {
                panic!(
                    "assertion failed: `(left ≈ right)`\n  left: `{:?}`,\n right: `{:?}`,\n epsilon: `{:?}`",
                    $a, $b, $epsilon
                );
            }
        };
    }
}
