use utils::input;

fn main() {
    let input = input::read_input();
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
            let mut pin = 0;
            for row in 0..height {
                if block[row][col] == '#' {
                    pin += 1;
                }
            }
            pins.push(pin);
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
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise(&input);
        assert_eq!(res, 3);
    }
}
