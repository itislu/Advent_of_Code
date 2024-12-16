use std::collections::{BinaryHeap, HashMap};
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

fn exercise1(input: &str) -> i64 {
    let map = Map::new(input);
    let path = dijkstra(&map).expect("No path to the goal found!");

    print_map_with_path(&map, &path);
    path[&map.goal].cost
}

fn print_map_with_path(map: &Map, path: &HashMap<Position, Visit>) {
    for row in &map.grid {
        for tile in row {
            if let Some(visit) = path.get(&tile.pos) {
                print!("{}", visit)
            } else {
                print!("{}", tile);
            }
        }
        println!();
    }
}

fn dijkstra(map: &Map) -> Option<HashMap<Position, Visit>> {
    let mut queue: BinaryHeap<Visit> = BinaryHeap::new();
    let mut visited: HashMap<State, Visit> = HashMap::new();

    let start = State::new(map.start, Direction::East);
    let first_visit = Visit::new(start, start, 0);
    queue.push(first_visit);
    visited.insert(start, first_visit);

    while let Some(cur) = queue.pop() {
        if cur.state.pos == map.goal {
            let mut path: HashMap<Position, Visit> = HashMap::new();
            let mut best = cur;

            while best.state.pos != map.start {
                path.insert(best.state.pos, best);
                best = visited[&best.came_from];
            }
            path.insert(best.state.pos, best);
            return Some(path);
        }

        for neighbor_visit in map.visits(&cur) {
            if !visited.contains_key(&neighbor_visit.state)
                || neighbor_visit.cost < visited[&neighbor_visit.state].cost
            {
                visited.insert(neighbor_visit.state, neighbor_visit);
                queue.push(neighbor_visit);
            }
        }
    }
    None
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn neighbors(&self) -> impl Iterator<Item = Self> {
        [
            Self::new(self.row - 1, self.col),
            Self::new(self.row + 1, self.col),
            Self::new(self.row, self.col + 1),
            Self::new(self.row, self.col - 1),
        ]
        .into_iter()
    }

    fn to(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self::new(self.row - 1, self.col),
            Direction::South => Self::new(self.row + 1, self.col),
            Direction::East => Self::new(self.row, self.col + 1),
            Direction::West => Self::new(self.row, self.col - 1),
        }
    }

    fn dir(&self, from: Position) -> Direction {
        let row_diff = self.row as i64 - from.row as i64;
        let col_diff = self.col as i64 - from.col as i64;

        if col_diff.abs() >= row_diff.abs() {
            match col_diff >= 0 {
                true => Direction::East,
                false => Direction::West,
            }
        } else {
            match row_diff <= 0 {
                true => Direction::North,
                false => Direction::South,
            }
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn counter_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => '^',
                Direction::South => 'v',
                Direction::East => '>',
                Direction::West => '<',
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Position,
    facing: Direction,
}

impl State {
    fn new(pos: Position, facing: Direction) -> Self {
        Self { pos, facing }
    }

    fn ahead(&self) -> Self {
        Self {
            pos: self.pos.to(self.facing),
            facing: self.facing,
        }
    }

    fn clockwise(&self) -> Self {
        Self {
            pos: self.pos,
            facing: self.facing.clockwise(),
        }
    }

    fn counter_clockwise(&self) -> Self {
        Self {
            pos: self.pos,
            facing: self.facing.counter_clockwise(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Visit {
    state: State,
    came_from: State,
    cost: i64,
}

impl Visit {
    fn new(state: State, came_from: State, cost: i64) -> Self {
        Self {
            state,
            came_from,
            cost,
        }
    }

    fn visits(&self) -> impl Iterator<Item = Visit> {
        [
            Visit::new(self.state.ahead(), self.state, self.cost + 1),
            Visit::new(self.state.clockwise(), self.state, self.cost + 1000),
            Visit::new(self.state.counter_clockwise(), self.state, self.cost + 1000),
        ]
        .into_iter()
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
        write!(f, "{}", self.state.facing)
    }
}

#[derive(PartialEq, Eq)]
enum TileKind {
    Wall,
    Path,
    Start,
    Goal,
}

impl From<char> for TileKind {
    fn from(c: char) -> Self {
        match c {
            '#' => TileKind::Wall,
            '.' => TileKind::Path,
            'S' => TileKind::Start,
            'E' => TileKind::Goal,
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
                TileKind::Wall => '#',
                TileKind::Path => '.',
                TileKind::Start => 'S',
                TileKind::Goal => 'E',
            }
        )
    }
}

struct Tile {
    pos: Position,
    kind: TileKind,
}

impl Tile {
    fn new(row: usize, col: usize, c: char) -> Self {
        Self {
            pos: Position::new(row, col),
            kind: TileKind::from(c),
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
}

impl Map {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<Tile>> = Vec::new();
        let mut start_opt: Option<Position> = None;
        let mut goal_opt: Option<Position> = None;

        for (row, line) in input.lines().enumerate() {
            let mut grid_line: Vec<Tile> = Vec::new();
            for (col, c) in line.chars().enumerate() {
                let tile = Tile::new(row, col, c);
                match tile.kind {
                    TileKind::Start => start_opt = Some(tile.pos),
                    TileKind::Goal => goal_opt = Some(tile.pos),
                    _ => {}
                };
                grid_line.push(tile);
            }
            grid.push(grid_line);
        }
        Self {
            grid,
            start: start_opt.expect("No start tile ('S') found!"),
            goal: goal_opt.expect("No goal tile ('E') found!"),
        }
    }

    fn visits<'a>(&'a self, cur_visit: &'a Visit) -> impl Iterator<Item = Visit> + 'a {
        cur_visit
            .visits()
            .filter(|visit| self.at(visit.state.pos).kind != TileKind::Wall)
    }

    fn at(&self, pos: Position) -> &Tile {
        &self.grid[pos.row][pos.col]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1_ex1() {
        let input = input::read_file("input_example1.txt");
        let res = exercise1(&input);
        assert_eq!(res, 7036);
    }

    #[test]
    fn test2_ex1() {
        let input = input::read_file("input_example2.txt");
        let res = exercise1(&input);
        assert_eq!(res, 11048);
    }
}
