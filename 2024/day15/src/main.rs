use std::collections::VecDeque;
use utils::input;

/*
    Warehouse Woes - Day 15
    Part 1: Track a robot (@) pushing boxes (O) around a warehouse according to movement instructions (^v<>).
            Calculate sum of box GPS coordinates (row * 100 + col) after all moves.
    Part 2: Same as part 1, but with double-width warehouse and boxes ([]). Robot stays same size.
*/

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let mut map = Map::new(input, false);
    let mut robot = Robot::new(&map, input);

    while !robot.movements.is_empty() {
        robot.mv(&mut map);
    }
    map.boxes().map(|object| object.gps_coordinate()).sum()
}

fn exercise2(input: &str) -> usize {
    let mut map = Map::new(input, true);
    let mut robot = Robot::new(&map, input);

    while !robot.movements.is_empty() {
        robot.mv(&mut map);
    }
    map.boxes().map(|object| object.gps_coordinate()).sum()
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        use Direction::*;
        match c {
            '^' => Up,
            'v' => Down,
            '>' => Right,
            '<' => Left,
            _ => panic!("Invalid character in movement found!"),
        }
    }
}

impl Direction {
    fn is_horizontal(&self) -> bool {
        *self == Self::Right || *self == Self::Left
    }

    fn is_vertical(&self) -> bool {
        *self == Self::Up || *self == Self::Down
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    fn to(&self, direction: Direction) -> Self {
        use Direction::*;
        match direction {
            Up => Self::new(self.row - 1, self.col),
            Down => Self::new(self.row + 1, self.col),
            Right => Self::new(self.row, self.col + 1),
            Left => Self::new(self.row, self.col - 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum BoxPart {
    Left,
    Right,
}

impl BoxPart {
    fn other(&self, pos: Position) -> Position {
        match self {
            BoxPart::Left => Position::new(pos.row, pos.col + 1),
            BoxPart::Right => Position::new(pos.row, pos.col - 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ObjectKind {
    Empty,
    Wall,
    Box(Option<BoxPart>),
    Robot,
}

struct Object {
    kind: ObjectKind,
    pos: Position,
}

impl Object {
    fn new(c: char, row: usize, col: usize) -> Self {
        let pos = Position::new(row, col);
        let kind = match c {
            '.' => ObjectKind::Empty,
            '#' => ObjectKind::Wall,
            'O' => ObjectKind::Box(None),
            '@' => ObjectKind::Robot,
            _ => panic!("Invalid character in map found!"),
        };
        Object { kind, pos }
    }

    fn twice(c: char, row: usize, col: usize) -> [Self; 2] {
        let (pos1, pos2) = (Position::new(row, col), Position::new(row, col + 1));
        let (kind1, kind2) = match c {
            '.' => (ObjectKind::Empty, ObjectKind::Empty),
            '#' => (ObjectKind::Wall, ObjectKind::Wall),
            'O' => (
                ObjectKind::Box(Some(BoxPart::Left)),
                ObjectKind::Box(Some(BoxPart::Right)),
            ),
            '@' => (ObjectKind::Robot, ObjectKind::Empty),
            _ => panic!("Invalid character in map found!"),
        };
        [
            Object {
                kind: kind1,
                pos: pos1,
            },
            Object {
                kind: kind2,
                pos: pos2,
            },
        ]
    }

    fn gps_coordinate(&self) -> usize {
        self.pos.row * 100 + self.pos.col
    }
}

struct Robot {
    pos: Position,
    movements: VecDeque<Direction>,
}

impl Robot {
    fn new(map: &Map, input: &str) -> Self {
        Self {
            pos: map
                .grid
                .iter()
                .flatten()
                .find(|object| object.kind == ObjectKind::Robot)
                .expect("No robot in map found!")
                .pos,
            movements: input
                .split("\n\n")
                .skip(1)
                .collect::<String>()
                .lines()
                .flat_map(|line| line.chars())
                .map(Direction::from)
                .collect(),
        }
    }

    fn mv(&mut self, map: &mut Map) -> bool {
        if let Some(direction) = self.movements.pop_front() {
            if map.mv_object(self.pos, direction) {
                self.pos = self.pos.to(direction);
                return true;
            }
        }
        false
    }
}

struct Map {
    grid: Vec<Vec<Object>>,
}

impl Map {
    fn new(input: &str, is_wide: bool) -> Self {
        let mut grid: Vec<Vec<Object>> = Vec::new();

        for (row, line) in input.split("\n\n").nth(0).unwrap().lines().enumerate() {
            let mut grid_line: Vec<Object> = Vec::new();
            for (mut col, c) in line.chars().enumerate() {
                if is_wide {
                    col *= 2;
                    grid_line.extend(Object::twice(c, row, col));
                } else {
                    grid_line.push(Object::new(c, row, col));
                }
            }
            grid.push(grid_line);
        }
        Self { grid }
    }

    fn can_move(&self, pos: Position, direction: Direction) -> bool {
        use ObjectKind::*;
        match self.at(pos).kind {
            Empty => true,
            Wall => false,
            Box(Some(part)) => {
                self.can_move(pos.to(direction), direction)
                    && (direction.is_horizontal()
                        || self.can_move(part.other(pos).to(direction), direction))
            }
            Box(None) | Robot => self.can_move(pos.to(direction), direction),
        }
    }

    fn mv_object(&mut self, pos: Position, direction: Direction) -> bool {
        use ObjectKind::*;

        if !self.can_move(pos, direction) {
            return false;
        }
        match self.at(pos).kind {
            Empty => true,
            Wall => false,
            Box(Some(part)) => {
                let new_pos = pos.to(direction);
                self.mv_object(new_pos, direction);
                self.swap(pos, new_pos);
                if direction.is_vertical() {
                    let other_new_pos = part.other(pos).to(direction);
                    self.mv_object(other_new_pos, direction);
                    self.swap(part.other(pos), other_new_pos);
                }
                true
            }
            Box(None) | Robot => {
                let new_pos = pos.to(direction);
                if self.mv_object(new_pos, direction) {
                    self.swap(pos, new_pos);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn swap(&mut self, pos1: Position, pos2: Position) {
        let tmp = self.at(pos1).kind;
        self.at_mut(pos1).kind = self.at(pos2).kind;
        self.at_mut(pos2).kind = tmp;
    }

    fn boxes(&self) -> impl Iterator<Item = &Object> {
        self.grid.iter().flatten().filter(|object| {
            matches!(
                object.kind,
                ObjectKind::Box(None) | ObjectKind::Box(Some(BoxPart::Left))
            )
        })
    }

    fn at(&self, pos: Position) -> &Object {
        &self.grid[pos.row][pos.col]
    }

    fn at_mut(&mut self, pos: Position) -> &mut Object {
        &mut self.grid[pos.row][pos.col]
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
            assert_eq!(res, 2028);
        }

        #[test]
        fn example() {
            let input = input::read_file("example.txt");
            let res = exercise1(&input);
            assert_eq!(res, 10092);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise1(&input);
            assert_eq!(res, 1568399);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input);
            assert_eq!(res, 9021);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise2(&input);
            assert_eq!(res, 1575877);
        }
    }
}
