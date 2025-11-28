/// Combines two unsigned integers, `a` and `b`, by appending the digits of `b` to `a`.
pub fn concat_nums(a: u64, b: u64) -> u64 {
    assert!(b > 0);
    a * 10u64.pow(b.ilog10() + 1) + b as u64
}

#[cfg(test)]
mod converts_tests {
    use super::*;

    #[test]
    fn test_concat_nums_basic() {
        assert_eq!(concat_nums(12, 34), 1234);
    }

    #[test]
    fn test_concat_nums_single_digit() {
        assert_eq!(concat_nums(7, 5), 75);
    }

    #[test]
    fn test_concat_nums_with_zero() {
        assert_eq!(concat_nums(0, 123), 123);
    }

    #[test]
    fn test_concat_nums_large_numbers() {
        assert_eq!(concat_nums(12345, 67890), 1234567890);
    }

    #[test]
    fn test_concat_nums_same_numbers() {
        assert_eq!(concat_nums(111, 111), 111111);
    }
}
