use itertools::Itertools;
use strum::IntoEnumIterator;
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let mut map = Map::new(input);

    for start in map.starts.clone() {
        let start_cell = map.at(&start).unwrap().clone();
        hike(&mut map, &start_cell, &start_cell);
    }
    map.trails.iter().unique().count()
}

fn exercise2(input: &str) -> usize {
    let mut map = Map::new(input);

    for start in map.starts.clone() {
        let start_cell = map.at(&start).unwrap().clone();
        hike(&mut map, &start_cell, &start_cell);
    }
    map.trails.len()
}

fn hike(map: &mut Map, start: &Cell, from: &Cell) {
    for direction in Direction::iter() {
        if let Some(to) = map.try_move(from, direction) {
            if to.value == 9 {
                map.trails.push((start.pos, to.pos));
            } else {
                hike(map, start, &to.clone());
            }
        }
    }
}

#[derive(strum_macros::EnumIter)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    fn to(&self, direction: Direction) -> Option<Self> {
        Some(match direction {
            Direction::Up => Position::new(self.row.checked_sub(1)?, self.col),
            Direction::Down => Position::new(self.row + 1, self.col),
            Direction::Right => Position::new(self.row, self.col + 1),
            Direction::Left => Position::new(self.row, self.col.checked_sub(1)?),
        })
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Clone)]
struct Cell {
    pos: Position,
    value: u8,
}

impl Cell {
    fn new(row: usize, col: usize, value: u8) -> Self {
        Cell {
            pos: Position::new(row, col),
            value,
        }
    }
}

struct Map {
    grid: Vec<Vec<Cell>>,
    starts: Vec<Position>,
    trails: Vec<(Position, Position)>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut starts: Vec<Position> = Vec::new();
        let grid: Vec<Vec<Cell>> = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| {
                        let digit = ch.to_digit(10).unwrap_or(u8::MAX as u32);
                        if digit == 0 {
                            starts.push(Position::new(row, col));
                        }
                        Cell::new(row, col, digit as u8)
                    })
                    .collect()
            })
            .collect();
        Map {
            starts,
            trails: Vec::new(),
            height: grid.len(),
            width: grid[0].len(),
            grid,
        }
    }

    fn try_move(&self, from: &Cell, direction: Direction) -> Option<&Cell> {
        let to = self.at(&from.pos.to(direction)?)?;
        if to.value == from.value + 1 {
            Some(to)
        } else {
            None
        }
    }

    fn is_in(&self, pos: &Position) -> bool {
        (0..self.height).contains(&pos.row) && (0..self.width).contains(&pos.col)
    }

    fn at(&self, pos: &Position) -> Option<&Cell> {
        if self.is_in(pos) {
            Some(&self.grid[pos.row][pos.col])
        } else {
            None
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", cell.value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 36);
    }

    #[test]
    fn test_ex2() {
        let input = input::read_example();
        let res = exercise2(&input);
        assert_eq!(res, 81);
    }
}
