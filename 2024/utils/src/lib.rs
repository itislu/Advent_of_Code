pub mod input {
    use std::env;
    use std::fs;
    use std::path;

    pub fn read_file(filename: &str) -> String {
        let dir = match env::var("CARGO_MANIFEST_DIR") {
            Ok(dir) => path::PathBuf::from(dir),
            Err(_) => env::current_dir().expect("Failed to get current directory"),
        };
        let path = dir.join(filename);
        fs::read_to_string(&path).expect(&format!("Failed to read file {}", path.display()))
    }
}

#[cfg(test)]
mod input_tests {
    use super::*;
    use input::*;

    #[test]
    fn print_read_file() {
        let result = read_file("Cargo.toml");
        println!("{}", result);
    }
}

pub mod parse {
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
}

#[cfg(test)]
mod parse_tests {
    use super::*;
    use parse::*;

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

pub mod colors {
    pub const RESET: &str = "\x1B[0m";
    pub const BOLD: &str = "\x1B[1m";

    pub const RED: &str = "\x1B[31m";
    pub const GREEN: &str = "\x1B[32m";
    pub const YELLOW: &str = "\x1B[33m";
    pub const BLUE: &str = "\x1B[34m";
    pub const MAGENTA: &str = "\x1B[35m";
    pub const CYAN: &str = "\x1B[36m";

    pub const BRIGHT_RED: &str = "\x1B[91m";
    pub const BRIGHT_GREEN: &str = "\x1B[92m";
    pub const BRIGHT_YELLOW: &str = "\x1B[93m";
    pub const BRIGHT_BLUE: &str = "\x1B[94m";
    pub const BRIGHT_MAGENTA: &str = "\x1B[95m";
    pub const BRIGHT_CYAN: &str = "\x1B[96m";

    pub const BOLD_RED: &str = "\x1B[1;31m";
    pub const BOLD_GREEN: &str = "\x1B[1;32m";
    pub const BOLD_YELLOW: &str = "\x1B[1;33m";
    pub const BOLD_BLUE: &str = "\x1B[1;34m";
    pub const BOLD_MAGENTA: &str = "\x1B[1;35m";
    pub const BOLD_CYAN: &str = "\x1B[1;36m";

    pub const BOLD_BRIGHT_RED: &str = "\x1B[1;91m";
    pub const BOLD_BRIGHT_GREEN: &str = "\x1B[1;92m";
    pub const BOLD_BRIGHT_YELLOW: &str = "\x1B[1;93m";
    pub const BOLD_BRIGHT_BLUE: &str = "\x1B[1;94m";
    pub const BOLD_BRIGHT_MAGENTA: &str = "\x1B[1;95m";
    pub const BOLD_BRIGHT_CYAN: &str = "\x1B[1;96m";
}
