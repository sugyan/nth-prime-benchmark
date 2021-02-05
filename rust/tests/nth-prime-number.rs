#[cfg(test)]
mod tests {
    fn test_cases() -> Vec<(usize, u32)> {
        vec![
            (1, 2),
            (2, 3),
            (3, 5),
            (4, 7),
            (5, 11),
            (6, 13),
            (7, 17),
            (8, 19),
            (9, 23),
            (10, 29),
            (11, 31),
            (12, 37),
            (13, 41),
            (14, 43),
            (15, 47),
            (16, 53),
            (17, 59),
            (18, 61),
            (19, 67),
            (20, 71),
            (21, 73),
            (22, 79),
            (23, 83),
            (24, 89),
            (25, 97),
            (50, 229),
            (100, 541),
            (1000, 7919),
            (10000, 104_729),
            (100000, 1_299_709),
        ]
    }

    #[test]
    fn test_trial_division() {
        for &(n, expected) in &test_cases() {
            assert_eq!(expected, nth_prime::trial_division(n));
        }
    }

    #[test]
    fn test_gmp() {
        for &(n, expected) in &test_cases() {
            assert_eq!(expected, nth_prime::gmp(n));
        }
    }

    #[test]
    fn test_eratosthenes() {
        for &(n, expected) in &test_cases() {
            assert_eq!(expected, nth_prime::eratosthenes(n));
        }
    }

    #[test]
    fn test_eratosthenes_pi() {
        for &(n, expected) in &test_cases() {
            assert_eq!(expected, nth_prime::eratosthenes_pi(n));
        }
    }

    #[test]
    fn test_eratosthenes_wf() {
        for &(n, expected) in &test_cases() {
            assert_eq!(expected, nth_prime::eratosthenes_wf(n));
        }
    }

    #[test]
    fn test_atkin() {
        for &(n, expected) in &test_cases() {
            assert_eq!(expected, nth_prime::atkin(n));
        }
    }

    #[test]
    fn test_primal() {
        for &(n, expected) in &test_cases() {
            assert_eq!(expected, nth_prime::primal(n));
        }
    }
}
