use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};
use utils::input;

/*
    Reindeer Maze - Day 16
    Part 1: Find lowest possible score to reach the end of a maze (1 point per step, 1000 points per turn).
    Part 2: Count how many tiles are part of any optimal (lowest-score) path through the maze.
*/

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> i64 {
    let map = Map::new(input);
    let path = dijkstra(&map).expect("No path to the goal found!");

    print_map_with_path(&map, &path);
    path[&map.goal].cost
}

fn exercise2(input: &str) -> usize {
    let map = Map::new(input);
    let best_paths = dijkstra_all(&map).expect("No path to the goal found!");

    print_map_with_path(&map, &best_paths);
    best_paths.len()
}

fn dijkstra(map: &Map) -> Option<HashMap<Position, Visit>> {
    let mut queue: BinaryHeap<Visit> = BinaryHeap::new();
    let mut visited: HashMap<State, Visit> = HashMap::new();

    let start = State::new(map.start, Direction::East);
    let first_visit = Visit::new(start, start, 0);
    queue.push(first_visit.clone());
    visited.insert(start, first_visit);

    while let Some(cur) = queue.pop() {
        if cur.state.pos == map.goal {
            return Some(collect_all_paths(&cur, &visited));
        }

        for neighbor_visit in map.visits(&cur) {
            if !visited.contains_key(&neighbor_visit.state)
                || neighbor_visit.cost < visited[&neighbor_visit.state].cost
            {
                visited.insert(neighbor_visit.state, neighbor_visit.clone());
                queue.push(neighbor_visit);
            }
        }
    }
    None
}

fn dijkstra_all(map: &Map) -> Option<HashMap<Position, Visit>> {
    let mut queue: BinaryHeap<Visit> = BinaryHeap::new();
    let mut visited: HashMap<State, Visit> = HashMap::new();

    let start = State::new(map.start, Direction::East);
    let first_visit = Visit::new(start, start, 0);
    queue.push(first_visit.clone());
    visited.insert(start, first_visit);

    while let Some(cur) = queue.pop() {
        if cur.state.pos == map.goal {
            return Some(collect_all_paths(&cur, &visited));
        }

        for neighbor_visit in map.visits(&cur) {
            match visited.get_mut(&neighbor_visit.state) {
                None => {
                    visited.insert(neighbor_visit.state, neighbor_visit.clone());
                    queue.push(neighbor_visit);
                }
                Some(existing) => match neighbor_visit.cost.cmp(&existing.cost) {
                    Ordering::Less => {
                        visited.insert(neighbor_visit.state, neighbor_visit.clone());
                        queue.push(neighbor_visit);
                    }
                    Ordering::Equal => {
                        existing.came_from.extend(neighbor_visit.came_from);
                    }
                    Ordering::Greater => (),
                },
            }
        }
    }
    None
}

fn collect_all_paths(visit: &Visit, visited: &HashMap<State, Visit>) -> HashMap<Position, Visit> {
    let mut best_paths: HashMap<Position, Visit> = HashMap::new();
    let mut to_process = vec![visit.clone()];

    while let Some(cur) = to_process.pop() {
        best_paths.insert(cur.state.pos, cur.clone());

        for prev_state in &cur.came_from {
            if let Some(prev_visit) = visited.get(prev_state) {
                if !best_paths.contains_key(&prev_visit.state.pos) {
                    to_process.push(prev_visit.clone());
                }
            }
        }
    }
    best_paths
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn to(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self::new(self.row - 1, self.col),
            Direction::South => Self::new(self.row + 1, self.col),
            Direction::East => Self::new(self.row, self.col + 1),
            Direction::West => Self::new(self.row, self.col - 1),
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct State {
    pos: Position,
    facing: Direction,
}

impl State {
    fn new(pos: Position, facing: Direction) -> Self {
        Self { pos, facing }
    }

    fn front(&self) -> Self {
        Self {
            pos: self.pos.to(self.facing),
            facing: self.facing,
        }
    }

    fn right(&self) -> Self {
        Self {
            pos: self.pos.to(self.facing.clockwise()),
            facing: self.facing.clockwise(),
        }
    }

    fn left(&self) -> Self {
        Self {
            pos: self.pos.to(self.facing.counter_clockwise()),
            facing: self.facing.counter_clockwise(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Visit {
    state: State,
    came_from: Vec<State>,
    cost: i64,
}

impl Visit {
    fn new(state: State, came_from: State, cost: i64) -> Self {
        Self {
            state,
            came_from: vec![came_from],
            cost,
        }
    }

    fn visits(&self) -> impl Iterator<Item = Visit> {
        [
            Visit::new(self.state.front(), self.state, self.cost + 1),
            Visit::new(self.state.right(), self.state, self.cost + 1001),
            Visit::new(self.state.left(), self.state, self.cost + 1001),
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
                    _ => (),
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
mod tests {
    use super::*;

    mod exercise1 {
        use super::*;

        #[test]
        fn example1() {
            let input = input::read_file("example1.txt");
            let res = exercise1(&input);
            assert_eq!(res, 7036);
        }

        #[test]
        fn example2() {
            let input = input::read_file("example2.txt");
            let res = exercise1(&input);
            assert_eq!(res, 11048);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise1(&input);
            assert_eq!(res, 89460);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn example1() {
            let input = input::read_file("example1.txt");
            let res = exercise2(&input);
            assert_eq!(res, 45);
        }

        #[test]
        fn example2() {
            let input = input::read_file("example2.txt");
            let res = exercise2(&input);
            assert_eq!(res, 64);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise2(&input);
            assert_eq!(res, 504);
        }
    }
}
