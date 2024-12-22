use std::{
    fmt,
    ops::{Add, Sub},
};
use utils::{input, parse};

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    // println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let mut res = 0;
    let mut numpad = NumPad::new();

    for line in input.lines() {
        let mut len: usize = 0;
        let num_code: Vec<NumPadKey> = line.chars().map(|c| NumPadKey::from(c)).collect();
        println!("{:?}", num_code);

        for button in num_code {
            println!("cur: {:?}, button: {:?}", numpad.current, button);
            len += control_dirpad(&numpad.press(button), 2);
        }

        println!("final len: {}\n", len);
        res += len * parse::numbers::<usize>(line).next().unwrap();
    }
    res
}

/*
Rule of Left only mattered at numpad level so far.
TODO: Try what value I get with consistently applying Rule of Left.
*/
fn exercise2(input: &str) -> usize {
    0
}

fn control_dirpad(dir_code: &[DirPadKey], indirections: u32) -> usize {
    #[cfg(debug_assertions)]
    println!(
        "indirections: {:2}, len: {}, {:?}",
        indirections,
        dir_code.len(),
        dir_code
    );

    if indirections == 0 {
        return dir_code.len();
    }
    let mut len = 0;
    let mut dirpad = DirPad::new();
    for &button in dir_code {
        len += control_dirpad(&dirpad.press(button), indirections - 1);
    }
    len
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

impl Add<DirPadKey> for Position {
    type Output = Position;

    fn add(self, dir: DirPadKey) -> Self::Output {
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

#[derive(Clone, Copy)]
enum NumPadKey {
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

impl NumPadKey {
    fn new(pos: Position) -> Option<NumPadKey> {
        use NumPadKey::*;
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

    fn pos(&self) -> Position {
        use NumPadKey::*;
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

    fn to(&self, dir: DirPadKey) -> Option<NumPadKey> {
        NumPadKey::new(self.pos() + dir)
    }

    fn is_allowed(&self, movements: &Vec<DirPadKey>) -> bool {
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

impl From<char> for NumPadKey {
    fn from(c: char) -> NumPadKey {
        use NumPadKey::*;
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

impl fmt::Display for NumPadKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use NumPadKey::*;
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

impl fmt::Debug for NumPadKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

struct NumPad {
    current: NumPadKey,
}

impl NumPad {
    fn new() -> NumPad {
        NumPad {
            current: NumPadKey::Activate,
        }
    }

    fn press(&mut self, target: NumPadKey) -> Vec<DirPadKey> {
        let mut movements: Vec<DirPadKey> = Vec::new();
        let row_diff = target.pos().row - self.current.pos().row;
        let col_diff = target.pos().col - self.current.pos().col;

        for _ in 0..col_diff.abs() {
            movements.push(if col_diff.is_positive() {
                DirPadKey::Right
            } else {
                DirPadKey::Left
            });
        }
        for _ in 0..row_diff.abs() {
            movements.push(if row_diff.is_positive() {
                DirPadKey::Up
            } else {
                DirPadKey::Down
            });
        }
        if col_diff.is_positive() {
            movements.reverse();
        }
        if !self.current.is_allowed(&movements) {
            movements.reverse();
        }
        movements.push(DirPadKey::Activate);
        self.current = target;
        movements
    }
}

#[derive(Clone, Copy)]
enum DirPadKey {
    Right,
    Up,
    Down,
    Left,
    Activate,
}

impl DirPadKey {
    fn new(pos: Position) -> Option<DirPadKey> {
        use DirPadKey::*;
        match pos {
            p if p == Right.pos() => Some(Right),
            p if p == Up.pos() => Some(Up),
            p if p == Down.pos() => Some(Down),
            p if p == Left.pos() => Some(Left),
            p if p == Activate.pos() => Some(Activate),
            _ => None,
        }
    }

    fn pos(&self) -> Position {
        use DirPadKey::*;
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

    fn dir(&self) -> Position {
        use DirPadKey::*;
        match self {
            Right => Position::new(0, 1),
            Up => Position::new(1, 0),
            Down => Position::new(-1, 0),
            Left => Position::new(0, -1),
            Activate => Position::new(0, 0),
        }
    }

    fn to(&self, dir: DirPadKey) -> Option<DirPadKey> {
        DirPadKey::new(self.pos() + dir)
    }

    fn is_allowed(&self, movements: &Vec<DirPadKey>) -> bool {
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

impl fmt::Display for DirPadKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DirPadKey::*;
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

impl fmt::Debug for DirPadKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

struct DirPad {
    current: DirPadKey,
}

impl DirPad {
    fn new() -> DirPad {
        DirPad {
            current: DirPadKey::Activate,
        }
    }

    fn press(&mut self, target: DirPadKey) -> Vec<DirPadKey> {
        let mut movements: Vec<DirPadKey> = Vec::new();
        let row_diff = target.pos().row - self.current.pos().row;
        let col_diff = target.pos().col - self.current.pos().col;

        for _ in 0..col_diff.abs() {
            movements.push(if col_diff.is_positive() {
                DirPadKey::Right
            } else {
                DirPadKey::Left
            });
        }
        for _ in 0..row_diff.abs() {
            movements.push(if row_diff.is_positive() {
                DirPadKey::Up
            } else {
                DirPadKey::Down
            });
        }
        if col_diff.is_positive() {
            movements.reverse();
        }
        if !self.current.is_allowed(&movements) {
            movements.reverse();
        }
        movements.push(DirPadKey::Activate);
        self.current = target;
        movements
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 126384);
    }

    // #[test]
    // fn test_ex2() {
    //     let input = input::read_example();
    //     let res = exercise2(&input);
    //     println!("{}", res);
    // }
}
