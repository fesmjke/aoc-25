/// Computes the greatest common divisor (GCD) of two non-negative integers.
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Computes the least common multiple (LCM) of two non-negative integers.
pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

/// Computes the least common multiple of a list of non-negative integers.
pub fn lcmm(numbers: &Vec<u64>) -> u64 {
    numbers.iter().copied().reduce(lcm).unwrap_or(1)
}

// https://www.rishabhxchoudhary.com/blog/overlapping-interval-problems
/// Merge intervals that overlap with each other into a consolidated set of non-overlapping intervals
pub fn merge_overlapping_ranges<T: PartialOrd + Ord + Copy>(ranges: Vec<(T, T)>) -> Vec<(T, T)> {
    let mut ranges = ranges.clone();

    ranges.sort_by_key(|interval| interval.0);

    let mut merged: Vec<(T, T)> = Vec::new();

    for &(start, end) in ranges.iter() {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    merged
}

#[cfg(test)]
mod algs_tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(101, 103), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 5), 20);
        assert_eq!(lcm(6, 8), 24);
        assert_eq!(lcm(7, 0), 0); // Any number multiplied by 0 results in 0
        assert_eq!(lcm(0, 7), 0);
    }

    #[test]
    fn test_lcmm() {
        assert_eq!(lcmm(&vec![4, 5, 6]), 60);
        assert_eq!(lcmm(&vec![7, 14, 21]), 42);
        assert_eq!(lcmm(&vec![2, 3, 5, 7, 11]), 2310);
        assert_eq!(lcmm(&vec![1]), 1);
        assert_eq!(lcmm(&vec![]), 1); // Edge case: empty vector
    }
}
