use regex::Regex;
use utils::input;

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> u64 {
    calculate(input)
}

fn exercise2(mut input: &str) -> u64 {
    let mut res: u64 = 0;
    let mut is_do: bool = true;
    let mut end: usize;

    while !input.is_empty() {
        if is_do {
            end = input.find("don't()").unwrap_or(input.len());
            res += calculate(&input[..end]);
        } else {
            end = input.find("do()").unwrap_or(input.len());
        }
        is_do = !is_do;
        input = &input[end..];
    }
    res
}

fn calculate(substr: &str) -> u64 {
    let re = Regex::new(r"mul\((?<n1>[+-]?\d+),(?<n2>[+-]?\d+)\)").unwrap();
    re.captures_iter(substr)
        .map(|caps| {
            caps.name("n1").unwrap().as_str().parse::<u64>().unwrap()
                * caps.name("n2").unwrap().as_str().parse::<u64>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod exercise1 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("example1.txt");
            let res = exercise1(&input);
            assert_eq!(res, 161);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise1(&input);
            assert_eq!(res, 157621318);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("example2.txt");
            let res = exercise2(&input);
            assert_eq!(res, 48);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise2(&input);
            assert_eq!(res, 79845780);
        }
    }
}
