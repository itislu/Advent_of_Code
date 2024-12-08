use itertools::Itertools;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Display},
    rc::Rc,
};
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &String) -> usize {
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

fn exercise2(input: &String) -> usize {
    let mut res: usize = 0;
    let mut map = Map::new(input);
    let mut antinodes = Vec::new();

    for antennas in map.antennas.values() {
        for combination in antennas.iter().combinations(2) {
            antinodes.extend(get_antinodes2(
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

fn get_antinodes1(antenna1: &Point, antenna2: &Point) -> Vec<Point> {
    let row_diff: i32 = antenna2.row - antenna1.row;
    let col_diff: i32 = antenna2.col - antenna1.col;
    vec![
        Point::new(antenna1.row - row_diff, antenna1.col - col_diff, '#'),
        Point::new(antenna2.row + row_diff, antenna2.col + col_diff, '#'),
    ]
}

fn get_antinodes2(antenna1: &Point, antenna2: &Point) -> Vec<Point> {
    let row_diff: i32 = antenna2.row - antenna1.row;
    let col_diff: i32 = antenna2.col - antenna1.col;

    (0..1000)
        .flat_map(|i| {
            [
                Point::new(
                    antenna1.row - row_diff * i,
                    antenna1.col - col_diff * i,
                    '#',
                ),
                Point::new(
                    antenna1.row + row_diff * i,
                    antenna1.col + col_diff * i,
                    '#',
                ),
            ]
        })
        .collect()
}

struct Map {
    grid: Vec<Vec<Rc<RefCell<Point>>>>,
    antennas: HashMap<char, Vec<Rc<RefCell<Point>>>>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(input: &String) -> Self {
        let mut grid: Vec<Vec<Rc<RefCell<Point>>>> = Vec::new();
        let mut antennas: HashMap<char, Vec<Rc<RefCell<Point>>>> = HashMap::new();

        for (i, line) in input.lines().enumerate() {
            let mut row: Vec<Rc<RefCell<Point>>> = Vec::new();
            for (j, ch) in line.chars().enumerate() {
                let point = Rc::new(RefCell::new(Point::new(i as i32, j as i32, ch)));
                if ch != '.' {
                    antennas
                        .entry(ch)
                        .or_insert(Vec::new())
                        .push(Rc::clone(&point));
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
        if self.is_in(&point) && !self.get(&point).borrow().contains('#') {
            self.get(&point).borrow_mut().data.push('#');
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
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 14);
    }

    #[test]
    fn test_ex2() {
        let input = input::read_example();
        let res = exercise2(&input);
        assert_eq!(res, 34);
    }
}
