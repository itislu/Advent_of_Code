use std::{
    collections::HashMap,
    fmt,
    ops::{Add, Sub},
};
use utils::{input, parse};

/*
    Keypad Conundrum - Day 21
    Part 1: Control a chain of 3 robots (using directional keypads) to input codes on a numeric keypad.
            Calculate sum of complexities (sequence_length * numeric_code) of the codes at the end of the chain.
    Part 2: Same as part 1, but with 26 robots in the chain instead of 3.
*/

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise(&input, 2));
    println!("exercise 2: {}", exercise(&input, 25));
}

fn exercise(input: &str, indirections: u32) -> usize {
    let mut res = 0;
    let mut numpad = KeyPad::new(NumKey::Activate);
    let mut cache: HashMap<(DirKey, DirKey, u32), usize> = HashMap::new();

    for line in input.lines() {
        let mut len: usize = 0;
        let num_code: Vec<NumKey> = line.chars().map(NumKey::from).collect();
        println!("{:?}", num_code);

        for button in num_code {
            println!("cur: {:?}, target: {:?}", numpad.current, button);
            len += remote_control(&numpad.press(button), &mut cache, indirections);
        }

        println!("final len: {}\n", len);
        res += len * parse::numbers::<usize>(line).next().unwrap();
    }
    res
}

fn remote_control(
    dir_code: &[DirKey],
    cache: &mut HashMap<(DirKey, DirKey, u32), usize>,
    indirections: u32,
) -> usize {
    #[cfg(all(debug_assertions, not(test)))]
    {
        println!(
            "indirections: {:2}, len: {}, {:?}",
            indirections,
            dir_code.len(),
            dir_code
        );
    }

    if indirections == 0 {
        return dir_code.len();
    }
    let mut len = 0;
    let mut dirpad = KeyPad::new(DirKey::Activate);

    for &button in dir_code {
        if let Some(cached_len) = cache.get(&(dirpad.current, button, indirections - 1)) {
            #[cfg(all(debug_assertions, not(test)))]
            {
                println!(
                    "CACHE HIT: (cur: {}, target: {}, indirections: {}), len: {}",
                    dirpad.current,
                    button,
                    indirections - 1,
                    cached_len
                );
            }
            dirpad.current = button;
            len += cached_len;
        } else {
            let cur = dirpad.current;
            let tmp = remote_control(&dirpad.press(button), cache, indirections - 1);
            cache.insert((cur, button, indirections - 1), tmp);
            len += tmp;
        }
    }
    len
}

struct KeyPad<T: Key> {
    current: T,
}

impl<T: Key> KeyPad<T> {
    fn new(initial: T) -> KeyPad<T> {
        KeyPad { current: initial }
    }

    fn press(&mut self, target: T) -> Vec<DirKey> {
        let mut movements: Vec<DirKey> = Vec::new();
        let row_diff = target.pos().row - self.current.pos().row;
        let col_diff = target.pos().col - self.current.pos().col;

        for _ in 0..col_diff.abs() {
            movements.push(if col_diff.is_positive() {
                DirKey::Right
            } else {
                DirKey::Left
            });
        }
        for _ in 0..row_diff.abs() {
            movements.push(if row_diff.is_positive() {
                DirKey::Up
            } else {
                DirKey::Down
            });
        }

        if col_diff.is_positive() {
            movements.reverse();
        }
        if !self.current.is_valid(&movements) {
            movements.reverse();
        }

        movements.push(DirKey::Activate);
        self.current = target;
        movements
    }
}

trait Key: Copy {
    fn pos(&self) -> Position;
    fn at(pos: Position) -> Option<Self>;
    fn to(&self, dir: DirKey) -> Option<Self> {
        Self::at(self.pos() + dir)
    }
    fn is_valid(&self, movements: &[DirKey]) -> bool {
        let mut cur = *self;
        for &movement in movements {
            if let Some(key) = cur.to(movement) {
                cur = key;
            } else {
                return false;
            }
        }
        true
    }
}

#[derive(Clone, Copy)]
enum NumKey {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
}

impl Key for NumKey {
    fn pos(&self) -> Position {
        use NumKey::*;
        let row: i8 = match self {
            Zero | Activate => 0,
            One | Two | Three => 1,
            Four | Five | Six => 2,
            Seven | Eight | Nine => 3,
        };
        let col: i8 = match self {
            One | Four | Seven => 0,
            Zero | Two | Five | Eight => 1,
            Activate | Three | Six | Nine => 2,
        };
        Position::new(row, col)
    }

    fn at(pos: Position) -> Option<NumKey> {
        use NumKey::*;
        match pos {
            p if p == Zero.pos() => Some(Zero),
            p if p == One.pos() => Some(One),
            p if p == Two.pos() => Some(Two),
            p if p == Three.pos() => Some(Three),
            p if p == Four.pos() => Some(Four),
            p if p == Five.pos() => Some(Five),
            p if p == Six.pos() => Some(Six),
            p if p == Seven.pos() => Some(Seven),
            p if p == Eight.pos() => Some(Eight),
            p if p == Nine.pos() => Some(Nine),
            p if p == Activate.pos() => Some(Activate),
            _ => None,
        }
    }
}

impl From<char> for NumKey {
    fn from(c: char) -> NumKey {
        use NumKey::*;
        match c {
            '0' => Zero,
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'A' => Activate,
            _ => panic!("Invalid NumPad character found!"),
        }
    }
}

impl fmt::Display for NumKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use NumKey::*;
        write!(
            f,
            "{}",
            match self {
                Zero => '0',
                One => '1',
                Two => '2',
                Three => '3',
                Four => '4',
                Five => '5',
                Six => '6',
                Seven => '7',
                Eight => '8',
                Nine => '9',
                Activate => 'A',
            }
        )
    }
}

impl fmt::Debug for NumKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum DirKey {
    Right,
    Up,
    Down,
    Left,
    Activate,
}

impl Key for DirKey {
    fn pos(&self) -> Position {
        use DirKey::*;
        let row: i8 = match self {
            Left | Down | Right => 0,
            Up | Activate => 1,
        };
        let col: i8 = match self {
            Left => 0,
            Up | Down => 1,
            Right | Activate => 2,
        };
        Position::new(row, col)
    }

    fn at(pos: Position) -> Option<DirKey> {
        use DirKey::*;
        match pos {
            p if p == Right.pos() => Some(Right),
            p if p == Up.pos() => Some(Up),
            p if p == Down.pos() => Some(Down),
            p if p == Left.pos() => Some(Left),
            p if p == Activate.pos() => Some(Activate),
            _ => None,
        }
    }
}

impl DirKey {
    fn dir(&self) -> Position {
        use DirKey::*;
        match self {
            Right => Position::new(0, 1),
            Up => Position::new(1, 0),
            Down => Position::new(-1, 0),
            Left => Position::new(0, -1),
            Activate => Position::new(0, 0),
        }
    }
}

impl fmt::Display for DirKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DirKey::*;
        write!(
            f,
            "{}",
            match self {
                Right => '>',
                Up => '^',
                Down => 'v',
                Left => '<',
                Activate => 'A',
            }
        )
    }
}

impl fmt::Debug for DirKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

#[derive(PartialEq, Eq)]
struct Position {
    row: i8,
    col: i8,
}

impl Position {
    fn new(row: i8, col: i8) -> Position {
        Position { row, col }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Self::Output {
        Position {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl Add<DirKey> for Position {
    type Output = Position;

    fn add(self, dir: DirKey) -> Self::Output {
        self + dir.dir()
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Self::Output {
        Position {
            row: self.row - other.row,
            col: self.col - other.col,
        }
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
            let res = exercise(&input, 2);
            assert_eq!(res, 126384);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise(&input, 2);
            assert_eq!(res, 219366);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise(&input, 25);
            assert_eq!(res, 271631192020464);
        }
    }
}
