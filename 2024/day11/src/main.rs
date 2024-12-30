use std::collections::{HashMap, HashSet};
use utils::input;

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let mut res: usize = 0;
    let stones: Vec<usize> = parse_stones(input);

    for stone in stones {
        res += split_stone_n_times(stone, 25);
    }
    res
}

fn exercise2(input: &str) -> usize {
    let mut res: usize = 0;
    let stones: Vec<usize> = parse_stones(input);
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    let mut tracker = Tracker::new();

    for stone in stones {
        let tmp = split_stone_n_times_cached(stone, 75, &mut cache, &mut tracker);
        cache.insert((stone, 75), tmp);
        res += tmp;
    }
    println!("\n{}", tracker);
    res
}

fn split_stone_n_times(mut stone: usize, mut n: usize) -> usize {
    let mut res: usize = 1;
    while n > 0 {
        if stone == 0 {
            stone = 1;
        } else if let Some((left, right)) = split_if_even_digits(stone) {
            res = split_stone_n_times(left, n - 1) + split_stone_n_times(right, n - 1);
            break;
        } else {
            stone *= 2024;
        }
        n -= 1;
    }
    res
}

fn split_stone_n_times_cached(
    mut stone: usize,
    mut n: usize,
    cache: &mut HashMap<(usize, usize), usize>,
    tracker: &mut Tracker,
) -> usize {
    if let Some(cached_result) = cache.get(&(stone, n)) {
        tracker.cache_hit(n);
        return *cached_result;
    } else {
        tracker.cache_miss();
    }
    let mut res: usize = 1;
    while n > 0 {
        if stone == 0 {
            stone = 1;
        } else if let Some((left, right)) = split_if_even_digits(stone) {
            tracker.unique_stones.insert(left);
            tracker.unique_stones.insert(right);
            let res_left = split_stone_n_times_cached(left, n - 1, cache, tracker);
            cache.insert((left, n - 1), res_left);
            let res_right = split_stone_n_times_cached(right, n - 1, cache, tracker);
            cache.insert((right, n - 1), res_right);
            res = res_left + res_right;
            break;
        } else {
            stone *= 2024;
        }
        tracker.unique_stones.insert(stone);
        n -= 1;
    }
    res
}

fn split_if_even_digits(num: usize) -> Option<(usize, usize)> {
    let string = num.to_string();
    if string.len() % 2 == 0 {
        let (left, right) = string.split_at(string.len() / 2);
        Some((left.parse().unwrap(), right.parse().unwrap()))
    } else {
        None
    }
}

fn parse_stones(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

struct Tracker {
    cache_hits: usize,
    cache_misses: usize,
    saved_iterations_total: usize,
    unique_stones: HashSet<usize>,
}

impl Tracker {
    fn new() -> Self {
        Tracker {
            cache_hits: 0,
            cache_misses: 0,
            saved_iterations_total: 0,
            unique_stones: HashSet::new(),
        }
    }

    fn cache_hit(&mut self, iter_left: usize) {
        self.cache_hits += 1;
        self.saved_iterations_total += iter_left;
    }

    fn cache_miss(&mut self) {
        self.cache_misses += 1;
    }
}

impl std::fmt::Display for Tracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "TRACKER:\ncache hits: {}\ncache misses: {}\nsaved iterations total: {}\nunique stones: {}",
            self.cache_hits,
            self.cache_misses,
            self.saved_iterations_total,
            self.unique_stones.len()
        )
    }
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
            assert_eq!(res, 55312);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise1(&input);
            assert_eq!(res, 183484);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise2(&input);
            assert_eq!(res, 218817038947400);
        }
    }
}
