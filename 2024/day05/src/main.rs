use std::cmp::Ordering;
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let rules: Vec<(u32, u32)> = parse_rules(input);
    let updates: Vec<Vec<u32>> = parse_updates(input);

    updates
        .iter()
        .filter(|update| {
            update
                .windows(2)
                .all(|window| is_correct_pair(window[0], window[1], &rules))
        })
        .map(|update| update[update.len() / 2] as usize)
        .sum()
}

fn exercise2(input: &str) -> usize {
    let rules: Vec<(u32, u32)> = parse_rules(input);
    let mut updates = parse_updates(input);
    let mut bad_updates: Vec<&mut Vec<u32>> = updates
        .iter_mut()
        .filter(|update| {
            update
                .windows(2)
                .any(|window| !is_correct_pair(window[0], window[1], &rules))
        })
        .collect();

    bad_updates
        .iter_mut()
        .for_each(|update| update.sort_by(|&a, &b| cmp_pair(a, b, &rules)));
    bad_updates
        .iter()
        .map(|update| update[update.len() / 2] as usize)
        .sum()
}

fn is_correct_pair(n1: u32, n2: u32, rules: &[(u32, u32)]) -> bool {
    rules.iter().any(|&rule| rule.0 == n1 && rule.1 == n2)
}

fn cmp_pair(n1: u32, n2: u32, rules: &[(u32, u32)]) -> Ordering {
    if is_correct_pair(n1, n2, rules) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn parse_rules(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map_while(|line| {
            if line.is_empty() {
                None
            } else {
                let nums: Vec<u32> = line.split('|').map(|n| n.parse::<u32>().unwrap()).collect();
                Some((nums[0], nums[1]))
            }
        })
        .collect()
}

fn parse_updates(input: &str) -> Vec<Vec<u32>> {
    if let Some(updates) = input.split("\n\n").nth(1) {
        updates
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect()
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 143);
    }

    #[test]
    fn test_ex2() {
        let input = input::read_example();
        let res = exercise2(&input);
        assert_eq!(res, 123);
    }
}
