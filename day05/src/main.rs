use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

fn exercise1(input: &String) -> usize {
    let mut res: usize = 0;
    let rules: Vec<(u32, u32)> = parse_rules(input);
    let updates: Vec<Vec<u32>> = parse_updates(input);

    for update in updates {
        if update
            .windows(2)
            .all(|window| is_correct_pair(window[0], window[1], &rules))
        {
            res += update[update.len() / 2] as usize;
        }
    }
    res
}

fn is_correct_pair(n1: u32, n2: u32, rules: &Vec<(u32, u32)>) -> bool {
    rules.iter().any(|&rule| rule.0 == n1 && rule.1 == n2)
}

fn parse_rules(input: &String) -> Vec<(u32, u32)> {
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

fn parse_updates(input: &String) -> Vec<Vec<u32>> {
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
}
