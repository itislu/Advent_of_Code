use utils::input;

/*
    Red-Nosed Reports - Day 2
    Part 1: Count sequences where numbers are strictly increasing/decreasing by 1-3.
    Part 2: Same as part 1, but allow removing one number to make sequence valid.
*/

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let mut res: usize = 0;
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        if is_close(&numbers) {
            res += 1;
        }
    }
    res
}

fn exercise2(input: &str) -> usize {
    let mut res: usize = 0;
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        for skip in 0..numbers.len() {
            let one_less: Vec<i32> = numbers
                .iter()
                .enumerate()
                .filter_map(|(i, &n)| if i != skip { Some(n) } else { None })
                .collect();
            if is_close(&one_less) {
                res += 1;
                break;
            }
        }
    }
    res
}

fn is_close(numbers: &[i32]) -> bool {
    for window in numbers.windows(2) {
        if (window[0] - window[1] < 0) != (numbers[0] - numbers[1] < 0)
            || window[0].abs_diff(window[1]) < 1
            || window[0].abs_diff(window[1]) > 3
        {
            return false;
        }
    }
    true
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
            assert_eq!(res, 2);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise1(&input);
            assert_eq!(res, 526);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input);
            assert_eq!(res, 4);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise2(&input);
            assert_eq!(res, 566);
        }
    }
}
