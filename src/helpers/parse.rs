use std::str::FromStr;

/// Parses a whitespace-separated string into a vector of values of type `T`.
pub fn parse_numbers_from_str<T>(nums: &str) -> Result<Vec<T>, T::Err>
where
    T: FromStr,
{
    nums.split(char::is_whitespace)
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.parse::<T>())
        .collect::<Result<Vec<T>, T::Err>>()
}

#[cfg(test)]
mod parse_tests {
    use super::parse_numbers_from_str;

    #[test]
    fn parse_numbers_from_str_i32() {
        let str = " 41 48 83 86 17 ";
        let expected = vec![41, 48, 83, 86, 17];
        let result = parse_numbers_from_str::<i32>(&str).unwrap();
        assert_eq!(expected, result);

        let str = " 83 86  6 31 17  9 48 53";
        let expected = vec![83, 86, 6, 31, 17, 9, 48, 53];
        let result = parse_numbers_from_str::<i32>(&str).unwrap();
        assert_eq!(expected, result);
    }
}
