use core::fmt;
use itertools::Itertools;
use std::{collections::HashMap, iter};
use utils::{colors, input};

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise1(&input, 100));
    println!("exercise 2: {}", exercise2(&input, 20, 100));
}

fn exercise1(input: &str, min_gain: usize) -> usize {
    const MAX_CHEAT: usize = 2;
    let racetrack = RaceTrack::new(input);

    racetrack
        .iter()
        .map(|cur_tile| {
            cur_tile
                .pos
                .distant_neighbors(MAX_CHEAT)
                .filter_map(|neighbor| racetrack.at(&neighbor))
                .filter(|cheat_tile| {
                    cheat_tile.time - cur_tile.time - MAX_CHEAT as i64 >= min_gain as i64
                })
                .count()
        })
        .sum()
}

fn exercise2(input: &str, max_cheat: usize, min_gain: usize) -> usize {
    let racetrack = RaceTrack::new(input);
    let mut cheats = 0;
    let mut _first_time = true;

    for cur_tile in racetrack.iter() {
        #[cfg(all(debug_assertions, not(test)))]
        let mut cheat_tiles: Vec<&TrackTile> = Vec::new();

        for _cheat_tile in cur_tile
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
            #[cfg(all(debug_assertions, not(test)))]
            {
                cheat_tiles.push(_cheat_tile);
            }
        }
        #[cfg(all(debug_assertions, not(test)))]
        {
            _print_track_with_cheat_tiles(&racetrack, cur_tile, &cheat_tiles, _first_time);
            _first_time = false;
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
        self.track.get(pos)
    }

    fn at_mut(&mut self, pos: &Position) -> Option<&mut TrackTile> {
        self.track.get_mut(pos)
    }

    fn iter(&self) -> impl Iterator<Item = &TrackTile> {
        iter::successors(self.at(&self.start), |tile| self.at(&tile.next?))
    }
}

fn _print_track_with_cheat_tiles(
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
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod exercise1 {
        use super::*;

        #[test]
        fn example_min2() {
            let input = input::read_file("example.txt");
            let res = exercise1(&input, 2);
            assert_eq!(res, 44);
        }

        #[test]
        fn example_min4() {
            let input = input::read_file("example.txt");
            let res = exercise1(&input, 4);
            assert_eq!(res, 30);
        }

        #[test]
        fn example_min6() {
            let input = input::read_file("example.txt");
            let res = exercise1(&input, 6);
            assert_eq!(res, 16);
        }

        #[test]
        fn example_min8() {
            let input = input::read_file("example.txt");
            let res = exercise1(&input, 8);
            assert_eq!(res, 14);
        }

        #[test]
        fn example_min10() {
            let input = input::read_file("example.txt");
            let res = exercise1(&input, 10);
            assert_eq!(res, 10);
        }

        #[test]
        fn example_min12() {
            let input = input::read_file("example.txt");
            let res = exercise1(&input, 12);
            assert_eq!(res, 8);
        }

        #[test]
        fn example_min20() {
            let input = input::read_file("example.txt");
            let res = exercise1(&input, 20);
            assert_eq!(res, 5);
        }

        #[test]
        fn example_min36() {
            let input = input::read_file("example.txt");
            let res = exercise1(&input, 36);
            assert_eq!(res, 4);
        }

        #[test]
        fn example_min38() {
            let input = input::read_file("example.txt");
            let res = exercise1(&input, 38);
            assert_eq!(res, 3);
        }

        #[test]
        fn example_min40() {
            let input = input::read_file("example.txt");
            let res = exercise1(&input, 40);
            assert_eq!(res, 2);
        }

        #[test]
        fn example_min64() {
            let input = input::read_file("example.txt");
            let res = exercise1(&input, 64);
            assert_eq!(res, 1);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise1(&input, 100);
            assert_eq!(res, 1445);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn example_min50() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input, 20, 50);
            assert_eq!(res, 285);
        }

        #[test]
        fn example_min52() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input, 20, 52);
            assert_eq!(res, 253);
        }

        #[test]
        fn example_min54() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input, 20, 54);
            assert_eq!(res, 222);
        }

        #[test]
        fn example_min56() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input, 20, 56);
            assert_eq!(res, 193);
        }

        #[test]
        fn example_min58() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input, 20, 58);
            assert_eq!(res, 154);
        }

        #[test]
        fn example_min60() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input, 20, 60);
            assert_eq!(res, 129);
        }

        #[test]
        fn example_min62() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input, 20, 62);
            assert_eq!(res, 106);
        }

        #[test]
        fn example_min64() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input, 20, 64);
            assert_eq!(res, 86);
        }

        #[test]
        fn example_min66() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input, 20, 66);
            assert_eq!(res, 67);
        }

        #[test]
        fn example_min68() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input, 20, 68);
            assert_eq!(res, 55);
        }

        #[test]
        fn example_min70() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input, 20, 70);
            assert_eq!(res, 41);
        }

        #[test]
        fn example_min72() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input, 20, 72);
            assert_eq!(res, 29);
        }

        #[test]
        fn example_min74() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input, 20, 74);
            assert_eq!(res, 7);
        }

        #[test]
        fn example_min76() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input, 20, 76);
            assert_eq!(res, 3);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise2(&input, 20, 100);
            assert_eq!(res, 1008040);
        }
    }
}
