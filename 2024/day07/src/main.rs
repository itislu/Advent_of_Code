use utils::input;

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

/*
Let's say we have 4 numbers - that means 3 spaces for operators.
How to get all permutations of what these 3 operators could be?
Well, there are 2^3 permutations possible. By counting down from 2^3 it will get all the possible bit patterns lower than the original pattern.
To read the bit pattern, just read all the bits of the punch card: a 1 means multiplication, a 0 addition.
*/
fn exercise1(input: &str) -> i64 {
    let mut res: i64 = 0;

    for line in input.lines() {
        let target = parse_target(line);
        let numbers = parse_numbers(line);

        if test_punch_card(0b1, target, &numbers) {
            res += target;
        }
    }
    res
}

fn exercise2(input: &str) -> i64 {
    let mut res: i64 = 0;

    for line in input.lines() {
        let target = parse_target(line);
        let numbers = parse_numbers(line);

        if test_punch_card(0b11, target, &numbers) {
            res += target;
        }
    }
    res
}

fn test_punch_card(mask: usize, target: i64, numbers: &Vec<i64>) -> bool {
    let mut punch_card: usize = (mask + 1).pow(numbers.len() as u32 - 1);

    loop {
        let operator = Operator::new(punch_card, mask, numbers);
        if operator.calculate() == target {
            return true;
        }
        if punch_card == 0 {
            return false;
        }
        punch_card -= 1;
    }
}

struct Operator<'a> {
    punch_card: usize,
    mask: usize,
    window: usize,
    numbers: &'a Vec<i64>,
}

impl<'a> Operator<'a> {
    fn new(punch_card: usize, mask: usize, numbers: &'a Vec<i64>) -> Self {
        Operator {
            punch_card,
            mask,
            window: (mask + 1) / 2,
            numbers,
        }
    }

    fn calculate(&self) -> i64 {
        if self.numbers.is_empty() {
            return 0;
        }
        let mut res: i64 = self.numbers[0];
        for (i, number) in self.numbers.iter().skip(1).enumerate() {
            match (self.punch_card >> (i * self.window)) & self.mask {
                0b00 => res += number,
                0b01 => res *= number,
                0b10 | 0b11 => res = concat(res, *number),
                _ => panic!("Unsupported punch card"),
            };
        }
        res
    }
}

fn concat(a: i64, b: i64) -> i64 {
    format!("{}{}", a, b).parse().unwrap()
}

fn parse_target(line: &str) -> i64 {
    line.split(':').next().unwrap().parse().unwrap()
}

fn parse_numbers(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect()
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
            assert_eq!(res, 3749);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise1(&input);
            assert_eq!(res, 4122618559853);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input);
            assert_eq!(res, 11387);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise2(&input);
            assert_eq!(res, 227615740238334);
        }
    }
}
