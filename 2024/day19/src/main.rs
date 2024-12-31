use std::collections::HashMap;
use utils::input;

/*
    Linen Layout - Day 19
    Part 1: Count how many towel designs are possible to make using given pattern pieces.
    Part 2: Count total number of different ways each possible design can be made.
*/

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let available: Vec<&str> = parse_available(input);
    let wanted: Vec<&str> = parse_wanted(input);
    let mut cache: HashMap<&str, bool> = HashMap::new();
    let mut possible: usize = 0;

    for design in wanted {
        let selected_available: Vec<&str> = available
            .iter()
            .filter(|&a| design.contains(a))
            .copied()
            .collect();
        if is_possible(design, &selected_available, &mut cache) {
            possible += 1;
        }
    }
    possible
}

fn exercise2(input: &str) -> usize {
    let available: Vec<&str> = parse_available(input);
    let wanted: Vec<&str> = parse_wanted(input);
    let mut cache: HashMap<&str, usize> = HashMap::new();
    let mut possible: usize = 0;

    for design in wanted {
        let selected_available: Vec<&str> = available
            .iter()
            .filter(|&a| design.contains(a))
            .copied()
            .collect();
        possible += count_possible(design, &selected_available, &mut cache);
    }
    possible
}

fn is_possible<'a>(
    wanted: &'a str,
    available: &Vec<&str>,
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(&cached_res) = cache.get(wanted) {
        return cached_res;
    }
    if wanted.is_empty() {
        return true;
    }
    for a in available {
        if let Some(substr) = wanted.strip_prefix(a) {
            if is_possible(substr, available, cache) {
                cache.insert(wanted, true);
                return true;
            }
        }
    }
    cache.insert(wanted, false);
    false
}

fn count_possible<'a>(
    wanted: &'a str,
    available: &Vec<&str>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&cached_res) = cache.get(wanted) {
        return cached_res;
    }
    if wanted.is_empty() {
        return 1;
    }
    let mut possible: usize = 0;

    for a in available {
        if let Some(substr) = wanted.strip_prefix(a) {
            possible += count_possible(substr, available, cache);
        }
    }
    cache.insert(wanted, possible);
    possible
}

fn parse_available(input: &str) -> Vec<&str> {
    input.lines().next().unwrap().split(", ").collect()
}

fn parse_wanted(input: &str) -> Vec<&str> {
    input.lines().skip(2).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod exercise1 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("example.txt");
            let res = exercise1(&input);
            assert_eq!(res, 6);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise1(&input);
            assert_eq!(res, 226);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input);
            assert_eq!(res, 16);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise2(&input);
            assert_eq!(res, 601201576113503);
        }
    }
}
