use std::ops::Not;
use utils::input;

/*
    Guard Gallivant - Day 6
    Part 1: Track path of guard following simple movement rules (turn right if blocked, else move forward)
            and count distinct positions visited before leaving map.
    Part 2: Find number of possible positions where placing an obstacle would trap guard in infinite loop.
*/

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let mut guard = Guard::new(input);

    println!("BEFORE:\n{}\n", guard.map);
    loop {
        if !guard.move_forward() {
            break;
        }
    }
    println!("AFTER:\n{}\n", guard.map);

    guard.map.count_visited()
}

fn exercise2(input: &str) -> usize {
    let mut res: usize = 0;
    let mut main_guard = Guard::new(input);

    loop {
        let mut trial_guard = main_guard.clone();
        if !main_guard.move_forward() {
            break;
        }
        if let Some(cell) = trial_guard.map.at(main_guard.row, main_guard.col) {
            if cell.indicator == Indicator::Visited {
                continue;
            }
        }
        trial_guard
            .map
            .put(main_guard.row, main_guard.col, Indicator::Obstacle);
        loop {
            match trial_guard.move_forward() {
                MoveResult::Success => (),
                MoveResult::InfiniteLoop => {
                    res += 1;
                    break;
                }
                MoveResult::OutOfBounds => break,
            }
        }
    }
    res
}

#[derive(Clone, Copy, PartialEq)]
enum Indicator {
    Start,
    Obstacle,
    Visited,
    NotVisited,
}

impl From<char> for Indicator {
    fn from(c: char) -> Self {
        match c {
            '^' => Indicator::Start,
            '#' => Indicator::Obstacle,
            'X' => Indicator::Visited,
            '.' => Indicator::NotVisited,
            _ => panic!("Invalid character in map found!"),
        }
    }
}

impl std::fmt::Display for Indicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Indicator::Start => '^',
            Indicator::Obstacle => '#',
            Indicator::Visited => 'X',
            Indicator::NotVisited => '.',
        };
        write!(f, "{}", c)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone)]
struct Cell {
    indicator: Indicator,
    row: i64,
    col: i64,
    visited: Vec<Direction>,
}

impl Cell {
    fn new(c: char, row: i64, col: i64) -> Self {
        Cell {
            indicator: Indicator::from(c),
            row,
            col,
            visited: Vec::new(),
        }
    }

    fn visit(&mut self, direction: Direction) -> Result<(), &Vec<Direction>> {
        self.visited.push(direction);
        if self.indicator == Indicator::Obstacle {
            Err(&self.visited)
        } else {
            self.indicator = Indicator::Visited;
            Ok(())
        }
    }
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<Cell>>,
    height: i64,
    width: i64,
}

impl Map {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<Cell>> = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| Cell::new(c, row as i64, col as i64))
                    .collect()
            })
            .collect();
        let height: i64 = grid.len() as i64;
        let width: i64 = grid[0].len() as i64;
        Map {
            grid,
            height,
            width,
        }
    }

    fn is_in(&self, row: i64, col: i64) -> bool {
        (0..self.height).contains(&row) && (0..self.width).contains(&col)
    }

    fn at(&mut self, row: i64, col: i64) -> Option<&mut Cell> {
        if self.is_in(row, col) {
            Some(&mut self.grid[row as usize][col as usize])
        } else {
            None
        }
    }

    fn put(&mut self, row: i64, col: i64, indicator: Indicator) -> bool {
        if self.is_in(row, col) {
            self.grid[row as usize][col as usize].indicator = indicator;
            true
        } else {
            false
        }
    }

    fn count_visited(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|&cell| cell.indicator == Indicator::Visited)
            .count()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map = self
            .grid
            .iter()
            .map(|line| {
                line.iter()
                    .map(|cell| cell.indicator.to_string())
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", map)
    }
}

#[derive(Clone, Copy)]
struct Movement {
    row_delta: i64,
    col_delta: i64,
    direction: Direction,
    alt_move: fn(&mut Guard) -> MoveResult,
}

enum MoveResult {
    Success,
    OutOfBounds,
    InfiniteLoop,
}

impl Not for MoveResult {
    type Output = bool;

    fn not(self) -> Self::Output {
        !matches!(self, MoveResult::Success)
    }
}

#[derive(Clone)]
struct Guard {
    map: Map,
    row: i64,
    col: i64,
    direction: Direction,
}

impl Guard {
    fn new(input: &str) -> Self {
        let mut map = Map::new(input);
        let mut row: i64 = -1;
        let mut col: i64 = -1;
        let direction = Direction::Up;

        if let Some(cell) = map
            .grid
            .iter_mut()
            .flatten()
            .find(|cell| cell.indicator == Indicator::Start)
        {
            row = cell.row;
            col = cell.col;
            cell.indicator = Indicator::Visited;
        }
        Guard {
            map,
            row,
            col,
            direction,
        }
    }

    fn move_forward(&mut self) -> MoveResult {
        match self.direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Right => self.move_right(),
            Direction::Left => self.move_left(),
        }
    }

    fn try_move(&mut self, movement: Movement) -> MoveResult {
        let new_row = self.row + movement.row_delta;
        let new_col = self.col + movement.col_delta;
        self.direction = movement.direction;

        if let Some(cell) = self.map.at(new_row, new_col) {
            match cell.visit(self.direction) {
                Ok(_) => {
                    self.row = new_row;
                    self.col = new_col;
                    MoveResult::Success
                }
                Err(visited) => {
                    if visited
                        .iter()
                        .filter(|&&direction| direction == self.direction)
                        .count()
                        <= 1
                    {
                        (movement.alt_move)(self)
                    } else {
                        MoveResult::InfiniteLoop
                    }
                }
            }
        } else {
            MoveResult::OutOfBounds
        }
    }

    fn move_up(&mut self) -> MoveResult {
        self.try_move(Movement {
            row_delta: -1,
            col_delta: 0,
            direction: Direction::Up,
            alt_move: Guard::move_right,
        })
    }

    fn move_right(&mut self) -> MoveResult {
        self.try_move(Movement {
            row_delta: 0,
            col_delta: 1,
            direction: Direction::Right,
            alt_move: Guard::move_down,
        })
    }

    fn move_down(&mut self) -> MoveResult {
        self.try_move(Movement {
            row_delta: 1,
            col_delta: 0,
            direction: Direction::Down,
            alt_move: Guard::move_left,
        })
    }

    fn move_left(&mut self) -> MoveResult {
        self.try_move(Movement {
            row_delta: 0,
            col_delta: -1,
            direction: Direction::Left,
            alt_move: Guard::move_up,
        })
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
            assert_eq!(res, 41);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise1(&input);
            assert_eq!(res, 4826);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input);
            assert_eq!(res, 6);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise2(&input);
            assert_eq!(res, 1721);
        }
    }
}
