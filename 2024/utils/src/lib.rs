pub mod input {
    use std::env;
    use std::fs;
    use std::io;
    use std::path;

    pub fn read_file(filename: &str) -> String {
        let dir = current_day_dir().expect("Failed to get directory of current day");
        let path = dir.join(filename);
        fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read file {}", path.display()))
    }

    fn current_day_dir() -> io::Result<path::PathBuf> {
        let cwd = env::current_dir()?;
        let exe = env::current_exe()?;
        let day = exe
            .file_name()
            .and_then(|f| f.to_str())
            .and_then(|s| s.get(0..5))
            .unwrap_or_default();
        let mut res = cwd.clone();

        loop {
            if let Some(component) = res.file_name() {
                if exe
                    .components()
                    .rev()
                    .skip(1)
                    .any(|d| d.as_os_str() == component)
                {
                    break;
                }
            }
            if !res.pop() {
                res = cwd;
                break;
            }
        }
        res.push(day);
        Ok(res)
    }

    #[allow(dead_code)]
    fn debug_paths() {
        // 1. CARGO_MANIFEST_DIR
        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            eprintln!("\n=== CARGO_MANIFEST_DIR ===");
            eprintln!("{}", manifest_dir);
        }

        // 2. PWD
        if let Ok(pwd) = env::var("PWD") {
            eprintln!("\n=== PWD ===");
            eprintln!("{}", pwd);
        }

        // 3. Current working directory
        if let Ok(cwd) = env::current_dir() {
            eprintln!("\n=== current_dir() ===");
            eprintln!("{:?}", cwd);
        }

        // 4. Executable path
        if let Ok(exe) = env::current_exe() {
            eprintln!("\n=== current_exe() ===");
            eprintln!("{:?}", exe);
        }

        // 5. Source file location
        eprintln!("\n=== file!() ===");
        eprintln!("{}", file!());

        // 6. Source absolute path
        if let Ok(canonical) = fs::canonicalize(file!()) {
            eprintln!("\n=== canonicalize(file!()) ===");
            eprintln!("{:?}", canonical);
        }

        // 7. Home directory
        if let Ok(home) = env::var("HOME") {
            eprintln!("\n=== HOME ===");
            eprintln!("{}", home);
        }

        // 8. OUT_DIR (only available during build)
        if let Ok(out_dir) = env::var("OUT_DIR") {
            eprintln!("\n=== OUT_DIR ===");
            eprintln!("{}", out_dir);
        }

        eprintln!();
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

// #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// enum Direction {
//     Up,
//     Down,
//     Right,
//     Left,
// }

// impl Direction {
//     fn clockwise(&self) -> Self {
//         match self {
//             Direction::Up => Direction::Right,
//             Direction::Right => Direction::Down,
//             Direction::Down => Direction::Left,
//             Direction::Left => Direction::Up,
//         }
//     }

//     fn counter_clockwise(&self) -> Self {
//         match self {
//             Direction::Up => Direction::Left,
//             Direction::Left => Direction::Down,
//             Direction::Down => Direction::Right,
//             Direction::Right => Direction::Up,
//         }
//     }
// }

// impl std::fmt::Display for Direction {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 Direction::Up => '^',
//                 Direction::Down => 'v',
//                 Direction::Right => '>',
//                 Direction::Left => '<',
//             }
//         )
//     }
// }

// #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
// struct Position {
//     row: usize,
//     col: usize,
// }

// impl Position {
//     fn new(row: usize, col: usize) -> Self {
//         Self { row, col }
//     }

//     fn neighbors(&self) -> impl Iterator<Item = Self> {
//         [
//             Self::new(self.row - 1, self.col),
//             Self::new(self.row + 1, self.col),
//             Self::new(self.row, self.col + 1),
//             Self::new(self.row, self.col - 1),
//         ]
//         .into_iter()
//     }

//     fn to(&self, direction: Direction) -> Self {
//         match direction {
//             Direction::Up => Self::new(self.row - 1, self.col),
//             Direction::Down => Self::new(self.row + 1, self.col),
//             Direction::Right => Self::new(self.row, self.col + 1),
//             Direction::Left => Self::new(self.row, self.col - 1),
//         }
//     }

//     fn dir(&self, from: Position) -> Direction {
//         let row_diff = if self.row >= from.row {
//             self.row - from.row
//         } else {
//             from.row - self.row
//         };
//         let col_diff = if self.col >= from.col {
//             self.col - from.col
//         } else {
//             from.col - self.col
//         };

//         if col_diff >= row_diff {
//             if self.col >= from.col {
//                 Direction::Right
//             } else {
//                 Direction::Left
//             }
//         } else {
//             if self.row <= from.row {
//                 Direction::Up
//             } else {
//                 Direction::Down
//             }
//         }
//     }
// }
