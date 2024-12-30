use itertools::Itertools;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Display},
    rc::Rc,
};
use utils::input;

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let mut res: usize = 0;
    let mut map = Map::new(input);
    let mut antinodes = Vec::new();

    for antennas in map.antennas.values() {
        for combination in antennas.iter().combinations(2) {
            /* `combination` is a `Vec<&Rc<RefCell<Point>>>`.
            `Rc` is a reference counter, `RefCell` a dynamic borrow checker.
            First index into the vector, then dereference the `Rc`, then use `borrow()` to borrow the value from the `RefCell`, and then pass a reference to that value.
            This would lead to `&*combination[0]`, but Rust is able to dereference this automatically. */
            antinodes.extend(get_antinodes1(
                &combination[0].borrow(),
                &combination[1].borrow(),
            ));
        }
    }
    for antinode in antinodes {
        if map.put_antinode(&antinode) {
            res += 1;
        }
    }
    println!("FINAL MAP:\n{}", map);
    res
}

fn exercise2(input: &str) -> usize {
    let mut res: usize = 0;
    let mut map = Map::new(input);
    let mut antinodes = Vec::new();

    for antennas in map.antennas.values() {
        for combination in antennas.iter().combinations(2) {
            antinodes.extend(get_antinodes2(
                &combination[0].borrow(),
                &combination[1].borrow(),
                &map,
            ));
        }
    }
    for antinode in antinodes {
        if map.put_antinode(&antinode) {
            res += 1;
        }
    }
    println!("FINAL MAP:\n{}", map);
    res
}

fn get_antinodes1(antenna1: &Point, antenna2: &Point) -> Vec<Point> {
    let row_diff: i32 = antenna2.row - antenna1.row;
    let col_diff: i32 = antenna2.col - antenna1.col;
    vec![
        Point::new(antenna1.row - row_diff, antenna1.col - col_diff, '#'),
        Point::new(antenna2.row + row_diff, antenna2.col + col_diff, '#'),
    ]
}

fn get_antinodes2(antenna1: &Point, antenna2: &Point, map: &Map) -> Vec<Point> {
    let row_diff = antenna2.row - antenna1.row;
    let col_diff = antenna2.col - antenna1.col;
    let mut antinodes = get_points_in_line(antenna1, row_diff, col_diff, map);
    antinodes.extend(get_points_in_line(antenna1, -row_diff, -col_diff, map));
    antinodes
}

fn get_points_in_line(start: &Point, row_diff: i32, col_diff: i32, map: &Map) -> Vec<Point> {
    let mut points = Vec::new();
    let mut row = start.row;
    let mut col = start.col;
    loop {
        let point = Point::new(row, col, '#');
        if !map.is_in(&point) {
            break;
        }
        points.push(point);
        row += row_diff;
        col += col_diff;
    }
    points
}

struct Map {
    grid: Vec<Vec<Rc<RefCell<Point>>>>,
    antennas: HashMap<char, Vec<Rc<RefCell<Point>>>>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<Rc<RefCell<Point>>>> = Vec::new();
        let mut antennas: HashMap<char, Vec<Rc<RefCell<Point>>>> = HashMap::new();

        for (i, line) in input.lines().enumerate() {
            let mut row: Vec<Rc<RefCell<Point>>> = Vec::new();
            for (j, ch) in line.chars().enumerate() {
                let point = Rc::new(RefCell::new(Point::new(i as i32, j as i32, ch)));
                if ch != '.' {
                    antennas.entry(ch).or_default().push(Rc::clone(&point));
                }
                row.push(point);
            }
            grid.push(row);
        }
        Map {
            height: grid.len(),
            width: grid[0].len(),
            grid,
            antennas,
        }
    }

    fn is_in(&self, point: &Point) -> bool {
        (0..self.height as i32).contains(&point.row) && (0..self.width as i32).contains(&point.col)
    }

    fn get(&self, point: &Point) -> &RefCell<Point> {
        &self.grid[point.row as usize][point.col as usize]
    }

    fn put_antinode(&mut self, point: &Point) -> bool {
        if self.is_in(point) && !self.get(point).borrow().contains('#') {
            self.get(point).borrow_mut().data.push('#');
            true
        } else {
            false
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for point in row {
                write!(f, "{}", point.borrow())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for point in self.grid.iter().flatten() {
            writeln!(f, "{:?}", point.borrow())?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Point {
    row: i32,
    col: i32,
    data: Vec<char>,
}

impl Point {
    fn new(row: i32, col: i32, data: char) -> Self {
        Point {
            row,
            col,
            data: vec![data],
        }
    }

    fn contains(&self, ch: char) -> bool {
        self.data.contains(&ch)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data.last().unwrap_or(&'_'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod exercise1 {
        use super::*;

        #[test]
        fn example1_1() {
            let input = input::read_file("example1_1.txt");
            let res = exercise1(&input);
            assert_eq!(res, 2);
        }

        #[test]
        fn example1_2() {
            let input = input::read_file("example1_2.txt");
            let res = exercise1(&input);
            assert_eq!(res, 4);
        }

        #[test]
        fn example1_3() {
            let input = input::read_file("example1_3.txt");
            let res = exercise1(&input);
            assert_eq!(res, 4);
        }

        #[test]
        fn example() {
            let input = input::read_file("example.txt");
            let res = exercise1(&input);
            assert_eq!(res, 14);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise1(&input);
            assert_eq!(res, 311);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn example2_1() {
            let input = input::read_file("example2_1.txt");
            let res = exercise2(&input);
            assert_eq!(res, 9);
        }

        #[test]
        fn example() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input);
            assert_eq!(res, 34);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise2(&input);
            assert_eq!(res, 1115);
        }
    }
}
