use core::fmt;
use itertools::Itertools;
use std::{collections::HashMap, iter};
use utils::{colors, input};

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise(&input, 2, 100));
    println!("exercise 2: {}", exercise(&input, 20, 100));
}

fn exercise(input: &str, max_cheat: usize, min_gain: usize) -> usize {
    let racetrack = RaceTrack::new(input);
    let mut cheats = 0;
    let mut first_time = true;

    for cur_tile in racetrack.iter() {
        #[cfg(debug_assertions)]
        let mut cheat_tiles: Vec<&TrackTile> = Vec::new();

        for cheat_tile in cur_tile
            .pos
            .circular_neighbors(max_cheat)
            .filter_map(|neighbor| racetrack.at(&neighbor))
            .filter(|cheat_tile| {
                cheat_tile.time
                    - cur_tile.time
                    - cur_tile.pos.manhattan_distance(cheat_tile.pos) as i64
                    >= min_gain as i64
            })
        {
            cheats += 1;
            #[cfg(debug_assertions)]
            cheat_tiles.push(cheat_tile);
        }
        #[cfg(debug_assertions)]
        {
            print_track_with_cheat_tiles(&racetrack, cur_tile, &cheat_tiles, first_time);
            first_time = false;
        }
    }
    cheats
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn neighbors(&self) -> impl Iterator<Item = Self> {
        self.distant_neighbors(1)
    }

    fn distant_neighbors(&self, distance: usize) -> impl Iterator<Item = Self> {
        let up = self
            .row
            .checked_sub(distance)
            .map(|row| Self::new(row, self.col));
        let down = Some(Self::new(self.row + distance, self.col));
        let right = Some(Self::new(self.row, self.col + distance));
        let left = self
            .col
            .checked_sub(distance)
            .map(|col| Self::new(self.row, col));

        [up, down, right, left].into_iter().flatten()
    }

    fn circular_neighbors(&self, distance: usize) -> impl Iterator<Item = Self> + '_ {
        itertools::repeat_n(-(distance as isize)..=distance as isize, 2)
            .multi_cartesian_product()
            .filter(move |diff| isize::abs(diff[0]) + isize::abs(diff[1]) <= distance as isize)
            .filter_map(|diff| {
                Some(Self::new(
                    self.row.checked_add_signed(diff[0])?,
                    self.col.checked_add_signed(diff[1])?,
                ))
            })
    }

    fn manhattan_distance(&self, other: Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

#[derive(Debug)]
struct TrackTile {
    pos: Position,
    time: i64,
    next: Option<Position>,
}

impl TrackTile {
    fn new(row: usize, col: usize) -> Self {
        Self {
            pos: Position::new(row, col),
            time: 0,
            next: None,
        }
    }
}

struct RaceTrack {
    track: HashMap<Position, TrackTile>,
    start: Position,
    finish: Position,
    height: usize,
    width: usize,
}

impl RaceTrack {
    fn new(input: &str) -> Self {
        let mut track: HashMap<Position, TrackTile> = HashMap::new();
        let mut start_opt: Option<Position> = None;
        let mut finish_opt: Option<Position> = None;

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start_opt = Some(Position::new(row, col));
                        track.insert(Position::new(row, col), TrackTile::new(row, col));
                    }
                    'E' => {
                        finish_opt = Some(Position::new(row, col));
                        track.insert(Position::new(row, col), TrackTile::new(row, col));
                    }
                    '.' => {
                        track.insert(Position::new(row, col), TrackTile::new(row, col));
                    }
                    _ => {}
                }
            }
        }
        let mut racetrack = Self {
            track,
            start: start_opt.expect("No start tile found!"),
            finish: finish_opt.expect("No end tile found!"),
            height: input.lines().count(),
            width: input.find('\n').unwrap(),
        };
        racetrack.build_track();
        racetrack
    }

    fn build_track(&mut self) {
        let mut cur = self.start;
        let mut prev = self.start;
        let mut time = 0;

        while let Some(next) = cur
            .neighbors()
            .filter_map(|neighbor| Some(self.at(&neighbor)?.pos))
            .find(|&pos| pos != prev)
        {
            let tile = self.at_mut(&cur).unwrap();
            tile.next = Some(next);
            tile.time = time;

            prev = cur;
            cur = next;
            time += 1;
        }
        self.at_mut(&cur).unwrap().time = time;
    }

    fn at(&self, pos: &Position) -> Option<&TrackTile> {
        self.track.get(&pos)
    }

    fn at_mut(&mut self, pos: &Position) -> Option<&mut TrackTile> {
        self.track.get_mut(&pos)
    }

    fn iter(&self) -> impl Iterator<Item = &TrackTile> {
        iter::successors(self.at(&self.start), |tile| self.at(&tile.next?))
    }
}

fn print_track_with_cheat_tiles(
    racetrack: &RaceTrack,
    cur_tile: &TrackTile,
    cheat_tiles: &Vec<&TrackTile>,
    first_time: bool,
) {
    let mut buffer: Vec<Vec<String>> =
        vec![vec![" ".to_string(); racetrack.width]; racetrack.height];

    for row in 0..racetrack.height {
        for col in 0..racetrack.width {
            if let Some(tile) = racetrack.track.get(&Position::new(row, col)) {
                if tile.pos == racetrack.start {
                    buffer[row][col] = colors::BOLD_BRIGHT_CYAN.to_string() + "S" + colors::RESET;
                } else if tile.pos == racetrack.finish {
                    buffer[row][col] = colors::BOLD_BRIGHT_CYAN.to_string() + "E" + colors::RESET;
                } else {
                    buffer[row][col] = ".".to_string();
                }
            } else {
                buffer[row][col] = "#".to_string();
            }
        }
    }
    buffer[cur_tile.pos.row][cur_tile.pos.col] =
        colors::BOLD_BRIGHT_YELLOW.to_string() + "I" + colors::RESET;
    for cheat_tile in cheat_tiles {
        buffer[cheat_tile.pos.row][cheat_tile.pos.col] =
            colors::BOLD_BRIGHT_GREEN.to_string() + "O" + colors::RESET;
    }

    if !first_time {
        print!("\x1B[{}A", racetrack.height);
    }
    println!(
        "{}",
        buffer
            .iter()
            .map(|row| row.join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

impl fmt::Display for RaceTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                if let Some(tile) = self.track.get(&Position::new(row, col)) {
                    if tile.pos == self.start {
                        write!(f, "S")?;
                    } else if tile.pos == self.finish {
                        write!(f, "E")?;
                    } else {
                        write!(f, ".")?;
                    }
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min2_ex1() {
        let input = input::read_example();
        let res = exercise(&input, 2, 2);
        assert_eq!(res, 44);
    }

    #[test]
    fn test_min4_ex1() {
        let input = input::read_example();
        let res = exercise(&input, 2, 4);
        assert_eq!(res, 30);
    }

    #[test]
    fn test_min6_ex1() {
        let input = input::read_example();
        let res = exercise(&input, 2, 6);
        assert_eq!(res, 16);
    }

    #[test]
    fn test_min8_ex1() {
        let input = input::read_example();
        let res = exercise(&input, 2, 8);
        assert_eq!(res, 14);
    }

    #[test]
    fn test_min10_ex1() {
        let input = input::read_example();
        let res = exercise(&input, 2, 10);
        assert_eq!(res, 10);
    }

    #[test]
    fn test_min12_ex1() {
        let input = input::read_example();
        let res = exercise(&input, 2, 12);
        assert_eq!(res, 8);
    }

    #[test]
    fn test_min20_ex1() {
        let input = input::read_example();
        let res = exercise(&input, 2, 20);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_min36_ex1() {
        let input = input::read_example();
        let res = exercise(&input, 2, 36);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_min38_ex1() {
        let input = input::read_example();
        let res = exercise(&input, 2, 38);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_min40_ex1() {
        let input = input::read_example();
        let res = exercise(&input, 2, 40);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_min64_ex1() {
        let input = input::read_example();
        let res = exercise(&input, 2, 64);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_min50_ex2() {
        let input = input::read_example();
        let res = exercise(&input, 20, 50);
        assert_eq!(res, 285);
    }

    #[test]
    fn test_min52_ex2() {
        let input = input::read_example();
        let res = exercise(&input, 20, 52);
        assert_eq!(res, 253);
    }

    #[test]
    fn test_min54_ex2() {
        let input = input::read_example();
        let res = exercise(&input, 20, 54);
        assert_eq!(res, 222);
    }

    #[test]
    fn test_min56_ex2() {
        let input = input::read_example();
        let res = exercise(&input, 20, 56);
        assert_eq!(res, 193);
    }

    #[test]
    fn test_min58_ex2() {
        let input = input::read_example();
        let res = exercise(&input, 20, 58);
        assert_eq!(res, 154);
    }

    #[test]
    fn test_min60_ex2() {
        let input = input::read_example();
        let res = exercise(&input, 20, 60);
        assert_eq!(res, 129);
    }

    #[test]
    fn test_min62_ex2() {
        let input = input::read_example();
        let res = exercise(&input, 20, 62);
        assert_eq!(res, 106);
    }

    #[test]
    fn test_min64_ex2() {
        let input = input::read_example();
        let res = exercise(&input, 20, 64);
        assert_eq!(res, 86);
    }

    #[test]
    fn test_min66_ex2() {
        let input = input::read_example();
        let res = exercise(&input, 20, 66);
        assert_eq!(res, 67);
    }

    #[test]
    fn test_min68_ex2() {
        let input = input::read_example();
        let res = exercise(&input, 20, 68);
        assert_eq!(res, 55);
    }

    #[test]
    fn test_min70_ex2() {
        let input = input::read_example();
        let res = exercise(&input, 20, 70);
        assert_eq!(res, 41);
    }

    #[test]
    fn test_min72_ex2() {
        let input = input::read_example();
        let res = exercise(&input, 20, 72);
        assert_eq!(res, 29);
    }

    #[test]
    fn test_min74_ex2() {
        let input = input::read_example();
        let res = exercise(&input, 20, 74);
        assert_eq!(res, 7);
    }

    #[test]
    fn test_min76_ex2() {
        let input = input::read_example();
        let res = exercise(&input, 20, 76);
        assert_eq!(res, 3);
    }
}
