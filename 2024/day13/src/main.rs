use itertools::Itertools;
use std::ops::{Add, Mul};
use utils::{input, parse};

/*
    Claw Contraption - Day 13
    Part 1: Find minimum tokens needed to win prizes by pressing A (3 tokens) and B (1 token) buttons that move claw by X,Y amounts to reach prize coordinates.
    Part 2: Same but with prize coordinates increased by 10^13 on both X and Y axes.
*/

const COST_A: i64 = 3;
const COST_B: i64 = 1;
const MAX_PRESSES: i64 = 100;
const GREAT_DISTANCE: i64 = 10000000000000;

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> i64 {
    let mut res: i64 = 0;
    for (button_a, button_b, prize) in parse_input(input) {
        if let Some(min_cost) = try_all_combinations(button_a, button_b, prize) {
            res += min_cost;
        }
    }
    res
}

fn exercise2(input: &str) -> i64 {
    let mut res: i64 = 0;
    for (mut button_a, mut button_b, prize) in parse_input(input) {
        if move_to_prize(&mut button_a, &mut button_b, prize + GREAT_DISTANCE) {
            res += button_a.get_cost() + button_b.get_cost();
        }
    }
    res
}

fn move_to_prize(button_a: &mut Button, button_b: &mut Button, prize: Position) -> bool {
    let ax = button_a.movement.x;
    let ay = button_a.movement.y;
    let bx = button_b.movement.x;
    let by = button_b.movement.y;
    let px = prize.x;
    let py = prize.y;

    let a = divide_if_whole(px * by - py * bx, ax * by - ay * bx);
    if a.is_none() {
        return false;
    }
    let b = divide_if_whole(px - ax * a.unwrap(), bx);
    if b.is_none() {
        return false;
    }
    button_a.presses = a.unwrap();
    button_b.presses = b.unwrap();
    true
}

fn divide_if_whole(a: i64, b: i64) -> Option<i64> {
    if a % b == 0 {
        Some(a / b)
    } else {
        None
    }
}

fn try_all_combinations(button_a: Button, button_b: Button, prize: Position) -> Option<i64> {
    let mut min_cost: Option<i64> = None;

    for combination in (0..=MAX_PRESSES).permutations(2) {
        if let Some(cost) = calc_cost(
            Button::new(
                button_a.movement.x,
                button_a.movement.y,
                combination[0],
                COST_A,
            ),
            Button::new(
                button_b.movement.x,
                button_b.movement.y,
                combination[1],
                COST_B,
            ),
            prize,
        ) {
            if min_cost.map_or(true, |min_cost| cost < min_cost) {
                min_cost = Some(cost);
            }
        }
    }
    min_cost
}

fn calc_cost(button_a: Button, button_b: Button, prize: Position) -> Option<i64> {
    if button_a.apply() + button_b.apply() == prize {
        Some(button_a.get_cost() + button_b.get_cost())
    } else {
        None
    }
}

struct Button {
    movement: Position,
    presses: i64,
    cost: i64,
}

impl Button {
    fn new(x: i64, y: i64, presses: i64, cost: i64) -> Self {
        Button {
            movement: Position::new(x, y),
            presses,
            cost,
        }
    }

    fn apply(&self) -> Position {
        self.movement * self.presses
    }

    fn get_cost(&self) -> i64 {
        self.presses * self.cost
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Position { x, y }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<i64> for Position {
    type Output = Position;

    fn add(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Mul<i64> for Position {
    type Output = Position;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = (Button, Button, Position)> + '_ {
    input.split("\n\n").map(|block| {
        let numbers: Vec<Vec<u64>> = block
            .lines()
            .map(|line| parse::numbers(line).collect())
            .collect();
        (
            Button::new(numbers[0][0] as i64, numbers[0][1] as i64, 0, COST_A),
            Button::new(numbers[1][0] as i64, numbers[1][1] as i64, 0, COST_B),
            Position::new(numbers[2][0] as i64, numbers[2][1] as i64),
        )
    })
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
            assert_eq!(res, 480);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise1(&input);
            assert_eq!(res, 27105);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise2(&input);
            assert_eq!(res, 101726882250942);
        }
    }
}
