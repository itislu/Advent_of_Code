use utils::input;

/*
    Code Chronicle - Day 25
    Part 1: Analyze lock/key schematics to find how many unique pairs fit together without overlap.
            Each lock/key is represented as columns of '#' extending down/up.
    Part 2: No actual puzzle - just a story conclusion. Free star after completing all previous 49 puzzles.
*/

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise: {}", exercise(&input));
}

fn exercise(input: &str) -> usize {
    let mut res = 0;
    let (locks, keys, height) = parse_locks_and_keys(input);

    for lock in &locks {
        for key in &keys {
            if lock.iter().zip(key.iter()).all(|(&l, &k)| l + k <= height) {
                res += 1;
            }
        }
    }
    res
}

fn parse_locks_and_keys(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>, u8) {
    let mut locks: Vec<Vec<u8>> = Vec::new();
    let mut keys: Vec<Vec<u8>> = Vec::new();
    let height = input.split("\n\n").next().unwrap().lines().count();
    let width = input.split("\n\n").next().unwrap().find('\n').unwrap();

    for block in input.split("\n\n").map(|block| {
        block
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>()
    }) {
        let mut pins: Vec<u8> = Vec::new();

        for col in 0..width {
            let pin = block.iter().filter(|row| row[col] == '#').count();
            pins.push(pin as u8);
        }

        if block[0][0] == '#' {
            locks.push(pins);
        } else {
            keys.push(pins);
        }
    }
    (locks, keys, height as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod exercise {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("example.txt");
            let res = exercise(&input);
            assert_eq!(res, 3);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise(&input);
            assert_eq!(res, 3116);
        }
    }
}
