use num_traits::Unsigned;
use std::str::FromStr;

pub fn numbers<T>(s: &str) -> impl Iterator<Item = T> + '_
where
    T: FromStr + Unsigned,
    T::Err: std::fmt::Debug,
{
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_zeros() {
        let nums: Vec<u32> = numbers("007 0042").collect();
        assert_eq!(nums, vec![7, 42]);
    }

    #[test]
    fn trailing_zeros() {
        let nums: Vec<u32> = numbers("700 4200").collect();
        assert_eq!(nums, vec![700, 4200]);
    }

    #[test]
    fn newlines() {
        let nums: Vec<u32> = numbers("12\n34\r\n56").collect();
        assert_eq!(nums, vec![12, 34, 56]);
    }

    #[test]
    fn mixed_content() {
        let nums: Vec<u32> = numbers("12ab34cd56").collect();
        assert_eq!(nums, vec![12, 34, 56]);
    }

    #[test]
    fn empty_string() {
        let nums: Vec<u32> = numbers("").collect();
        assert!(nums.is_empty());
    }

    #[test]
    fn no_numbers() {
        let nums: Vec<u32> = numbers("abc def").collect();
        assert!(nums.is_empty());
    }
}
