#![allow(unused_imports)]
use std::{
    cmp::min,
    collections::{BinaryHeap, HashMap, VecDeque},
    thread,
    time::Duration,
};
use strum::IntoEnumIterator;
use utils::{colors, input};

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise1(&input, 1024));
    println!(
        "exercise 2: {}",
        exercise2(&input).expect("No obstacle prevents the exit from being reachable.")
    );
}

fn exercise1(input: &str, obstacle_amount: usize) -> i64 {
    let mut obstacles: VecDeque<Position> = parse_obstacles(input);
    let rows = obstacles.iter().map(|pos| pos.row).max().unwrap() + 1;
    let cols = obstacles.iter().map(|pos| pos.col).max().unwrap() + 1;
    let mut map = Map::new(rows, cols);

    for obstacle in obstacles.drain(0..min(obstacle_amount, obstacles.len())) {
        map.put(obstacle, TileKind::Obstacle);
    }

    let path = dijkstra(&map).expect("No path to the goal found!");
    #[cfg(all(debug_assertions, not(test)))]
    {
        _print_map_with_path(&map, &path, true);
    }
    path[&map.goal].cost
}

fn exercise2(input: &str) -> Option<String> {
    let mut res: Option<Position> = None;
    let mut obstacles: VecDeque<Position> = parse_obstacles(input);
    let rows = obstacles.iter().map(|pos| pos.row).max().unwrap() + 1;
    let cols = obstacles.iter().map(|pos| pos.col).max().unwrap() + 1;
    let mut map = Map::new(rows, cols);
    let mut _first_time = true;

    while let Some(obstacle) = obstacles.pop_front() {
        map.put(obstacle, TileKind::Obstacle);
        if let Some(_path) = dijkstra(&map) {
            #[cfg(all(debug_assertions, not(test)))]
            {
                _print_map_with_path(&map, &_path, _first_time);
                _first_time = false;
                thread::sleep(Duration::from_millis(10))
            }
        } else {
            res = Some(obstacle);
            break;
        }
    }
    Some(format!("{},{}", res?.col, res?.row))
}

fn dijkstra(map: &Map) -> Option<HashMap<Position, Visit>> {
    let mut queue: BinaryHeap<Visit> = BinaryHeap::new();
    let mut visited: HashMap<Position, Visit> = HashMap::new();

    let start = Visit::new(map.start, map.start, 0);
    queue.push(start);
    visited.insert(map.start, start);

    while let Some(cur) = queue.pop() {
        if cur.pos == map.goal {
            let mut path: HashMap<Position, Visit> = HashMap::new();
            let mut current = cur;

            while current.pos != map.start {
                path.insert(current.pos, current);
                current = visited[&current.came_from];
            }
            path.insert(map.start, current);
            return Some(path);
        }

        for neighbor_visit in map
            .neighbors(cur.pos)
            .filter_map(|neighbor| cur.visit(neighbor))
        {
            if !visited.contains_key(&neighbor_visit.pos)
                || neighbor_visit.cost < visited[&neighbor_visit.pos].cost
            {
                visited.insert(neighbor_visit.pos, neighbor_visit);
                queue.push(neighbor_visit);
            }
        }
    }
    None
}

fn _print_map_with_path(map: &Map, path: &HashMap<Position, Visit>, first_time: bool) {
    let mut buffer = String::with_capacity(map.grid.len() * map.grid[0].len() * 2);

    for row in &map.grid {
        for tile in row {
            if let Some(visit) = path.get(&tile.pos) {
                buffer += &visit.to_string();
            } else {
                buffer += &tile.to_string();
            }
        }
        buffer += "\n";
    }
    if !first_time {
        print!("\x1B[{}A", map.height);
    }
    print!("{}", buffer);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn to(&self, direction: Direction) -> Option<Self> {
        Some(match direction {
            Direction::Up => Position::new(self.row.checked_sub(1)?, self.col),
            Direction::Down => Position::new(self.row + 1, self.col),
            Direction::Right => Position::new(self.row, self.col + 1),
            Direction::Left => Position::new(self.row, self.col.checked_sub(1)?),
        })
    }

    fn dir(&self, from: Position) -> Direction {
        let row_diff = self.row as i64 - from.row as i64;
        let col_diff = self.col as i64 - from.col as i64;

        if col_diff.abs() >= row_diff.abs() {
            match col_diff >= 0 {
                true => Direction::Right,
                false => Direction::Left,
            }
        } else {
            match row_diff <= 0 {
                true => Direction::Up,
                false => Direction::Down,
            }
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(PartialEq, Eq)]
enum TileKind {
    Obstacle,
    Free,
}

impl From<char> for TileKind {
    fn from(c: char) -> Self {
        match c {
            '#' => TileKind::Obstacle,
            '.' => TileKind::Free,
            _ => panic!("Invalid character in map found!"),
        }
    }
}

impl std::fmt::Display for TileKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TileKind::Obstacle => '#',
                TileKind::Free => '.',
            }
        )
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, strum_macros::EnumIter)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => format!("{}^{}", colors::BOLD_BRIGHT_YELLOW, colors::RESET),
                Direction::Down => format!("{}v{}", colors::BOLD_BRIGHT_YELLOW, colors::RESET),
                Direction::Right => format!("{}>{}", colors::BOLD_BRIGHT_YELLOW, colors::RESET),
                Direction::Left => format!("{}<{}", colors::BOLD_BRIGHT_YELLOW, colors::RESET),
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Visit {
    pos: Position,
    facing: Direction,
    came_from: Position,
    cost: i64,
}

impl Visit {
    fn new(pos: Position, came_from: Position, cost: i64) -> Self {
        Self {
            pos,
            facing: pos.dir(came_from),
            came_from,
            cost,
        }
    }

    fn visit(&self, target: &Tile) -> Option<Visit> {
        if target.kind == TileKind::Free {
            Some(Visit::new(target.pos, self.pos, self.cost + 1))
        } else {
            None
        }
    }
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Visit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.facing)
    }
}

struct Tile {
    pos: Position,
    kind: TileKind,
}

impl Tile {
    fn new(row: usize, col: usize, kind: TileKind) -> Self {
        Self {
            pos: Position::new(row, col),
            kind,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

struct Map {
    grid: Vec<Vec<Tile>>,
    start: Position,
    goal: Position,
    height: usize,
    width: usize,
}

impl Map {
    fn new(rows: usize, cols: usize) -> Self {
        let grid: Vec<Vec<Tile>> = (0..rows)
            .map(|row| {
                (0..cols)
                    .map(|col| Tile::new(row, col, TileKind::Free))
                    .collect()
            })
            .collect();
        Self {
            grid,
            start: Position::new(0, 0),
            goal: Position::new(rows - 1, cols - 1),
            height: rows,
            width: cols,
        }
    }

    fn neighbor(&self, pos: Position, direction: Direction) -> Option<&Tile> {
        if let Some(new_pos) = pos.to(direction) {
            self.at(&new_pos)
        } else {
            None
        }
    }

    fn neighbors(&self, pos: Position) -> impl Iterator<Item = &Tile> {
        Direction::iter().filter_map(move |direction| self.neighbor(pos, direction))
    }

    fn is_in(&self, pos: &Position) -> bool {
        (0..self.height).contains(&pos.row) && (0..self.width).contains(&pos.col)
    }

    fn at(&self, pos: &Position) -> Option<&Tile> {
        if self.is_in(pos) {
            Some(&self.grid[pos.row][pos.col])
        } else {
            None
        }
    }

    fn at_mut(&mut self, pos: &Position) -> Option<&mut Tile> {
        if self.is_in(pos) {
            Some(&mut self.grid[pos.row][pos.col])
        } else {
            None
        }
    }

    fn put(&mut self, pos: Position, kind: TileKind) -> bool {
        if let Some(tile) = self.at_mut(&pos) {
            tile.kind = kind;
            true
        } else {
            false
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_obstacles(input: &str) -> VecDeque<Position> {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(',')
                .map(|(col, row)| Position::new(row.parse().unwrap(), col.parse().unwrap()))
        })
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
            let res = exercise1(&input, 12);
            assert_eq!(res, 22);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise1(&input, 1024);
            assert_eq!(res, 334);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("example.txt");
            let res =
                exercise2(&input).expect("No obstacle prevents the exit from being reachable.");
            assert_eq!(res, "6,1");
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res =
                exercise2(&input).expect("No obstacle prevents the exit from being reachable.");
            assert_eq!(res, "20,12");
        }
    }
}
