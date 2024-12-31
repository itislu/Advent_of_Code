use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};
use strum::IntoEnumIterator;
use strum_macros::Display;
use utils::input;

/*
    Garden Groups - Day 12
    Part 1: Calculate total price of fencing garden regions by multiplying each region's area by its perimeter.
    Part 2: Same as part 1, but instead of perimeter, use total number of distinct fence sides regardless of length.
*/

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let map = Map::new(input);

    map.regions
        .iter()
        .map(|region| region.area * region.perimiter)
        .sum()
}

fn exercise2(input: &str) -> usize {
    let map = Map::new(input);

    map.regions
        .iter()
        .map(|region| region.area * region.sides)
        .sum()
}

#[derive(strum_macros::EnumIter, Clone, Copy, Display)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn side_directions(&self) -> impl Iterator<Item = Direction> {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        }
        .into_iter()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    fn to(&self, direction: Direction) -> Option<Self> {
        Some(match direction {
            Direction::Up => Position::new(self.row.checked_sub(1)?, self.col),
            Direction::Down => Position::new(self.row + 1, self.col),
            Direction::Right => Position::new(self.row, self.col + 1),
            Direction::Left => Position::new(self.row, self.col.checked_sub(1)?),
        })
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Clone)]
struct Tile {
    pos: Position,
    tile_type: char,
}

impl Tile {
    fn new(row: usize, col: usize, tile_type: char) -> Self {
        Tile {
            pos: Position::new(row, col),
            tile_type,
        }
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tile_type)
    }
}

struct Region {
    tiles: HashMap<Position, Rc<RefCell<Tile>>>,
    tile_type: char,
    area: usize,
    perimiter: usize,
    sides: usize,
}

impl Region {
    fn new(tiles: HashMap<Position, Rc<RefCell<Tile>>>, map: &Map) -> Self {
        let tile_type = tiles.values().nth(0).unwrap().borrow().tile_type;
        let area = tiles.len();
        let perimiter: usize = tiles
            .values()
            .map(|tile| {
                4 - map
                    .get_neighbours(&tile.borrow())
                    .iter()
                    .filter(|neighbour| neighbour.borrow().tile_type == tile_type)
                    .count()
            })
            .sum();
        let sides = Region::count_sides(&tiles, map);
        Region {
            tiles,
            tile_type,
            area,
            perimiter,
            sides,
        }
    }

    fn count_sides(tiles: &HashMap<Position, Rc<RefCell<Tile>>>, map: &Map) -> usize {
        let mut sides: usize = 0;

        for direction in Direction::iter() {
            let mut seen: HashSet<Position> = HashSet::from_iter(tiles.keys().cloned());

            for tile in tiles.values().map(|tile| tile.borrow()) {
                if seen.remove(&tile.pos) && Region::is_edge(&tile, direction, tiles, map) {
                    sides += 1;
                    // println!(
                    //     "found new side for: {}, at: {}, direction: {}",
                    //     tile.tile_type, tile.pos, direction
                    // );

                    for side_direction in direction.side_directions() {
                        let mut cur = tile.clone();

                        while let Some(neighbour) = map.get_neighbour(&cur, side_direction) {
                            let neighbour = neighbour.as_ref().borrow();
                            if !Region::is_edge(&neighbour, direction, tiles, map)
                                || !seen.remove(&neighbour.pos)
                            {
                                break;
                            }
                            cur = neighbour.clone();
                        }
                    }
                }
            }
        }
        sides
    }

    fn is_edge(
        tile: &Tile,
        direction: Direction,
        tiles: &HashMap<Position, Rc<RefCell<Tile>>>,
        map: &Map,
    ) -> bool {
        let potential_neighbour = map.get_neighbour(tile, direction);
        potential_neighbour.is_none()
            || !tiles.contains_key(&potential_neighbour.unwrap().borrow().pos)
    }

    fn collect(
        map: &Map,
        pos: &Position,
        tiles_without_regions: &mut HashSet<Position>,
    ) -> Option<HashMap<Position, Rc<RefCell<Tile>>>> {
        if let Some(tile_rc) = map.at(pos) {
            let tile = tile_rc.as_ref().borrow();
            if tiles_without_regions.remove(&tile.pos) {
                let mut region_tiles: HashMap<Position, Rc<RefCell<Tile>>> = HashMap::new();
                region_tiles.insert(tile.pos, Rc::clone(tile_rc));

                for neighbour_rc in map.get_neighbours(&tile) {
                    let neighbour = neighbour_rc.as_ref().borrow();
                    if neighbour.tile_type == tile.tile_type {
                        if let Some(new_tiles) =
                            Region::collect(map, &neighbour.pos, tiles_without_regions)
                        {
                            region_tiles.extend(new_tiles);
                        }
                    }
                }
                Some(region_tiles)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} * {}, area: {}, perimiter: {}, sides: {}",
            self.tiles.len(),
            self.tile_type,
            self.area,
            self.perimiter,
            self.sides,
        )
    }
}

struct Map {
    grid: Vec<Vec<Rc<RefCell<Tile>>>>,
    tiles_without_regions: HashSet<Position>,
    regions: Vec<Region>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut map = Map {
            grid: Vec::new(),
            tiles_without_regions: HashSet::new(),
            regions: Vec::new(),
            height: 0,
            width: 0,
        };
        let mut tiles_without_regions: HashSet<Position> = HashSet::new();
        let grid: Vec<Vec<Rc<RefCell<Tile>>>> = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| {
                        let tile = Rc::new(RefCell::new(Tile::new(row, col, ch)));
                        tiles_without_regions.insert(tile.as_ref().borrow().pos);
                        tile
                    })
                    .collect()
            })
            .collect();
        map.height = grid.len();
        map.width = grid[0].len();
        map.grid = grid;
        for tile in map
            .grid
            .iter()
            .flatten()
            .map(|tile_rc| tile_rc.as_ref().borrow())
        {
            if let Some(region_tiles) = Region::collect(&map, &tile.pos, &mut tiles_without_regions)
            {
                let region = Region::new(region_tiles, &map);
                // println!("new region: {}", region);
                map.regions.push(region);
            }
        }
        map.tiles_without_regions = tiles_without_regions;
        map
    }

    fn get_neighbours(&self, tile: &Tile) -> Vec<&Rc<RefCell<Tile>>> {
        let mut neighbours: Vec<&Rc<RefCell<Tile>>> = Vec::new();

        for direction in Direction::iter() {
            if let Some(neighbour) = self.get_neighbour(tile, direction) {
                neighbours.push(neighbour);
            }
        }
        neighbours
    }

    fn get_neighbour(&self, tile: &Tile, direction: Direction) -> Option<&Rc<RefCell<Tile>>> {
        if let Some(new_pos) = tile.pos.to(direction) {
            self.at(&new_pos)
        } else {
            None
        }
    }

    fn is_in(&self, pos: &Position) -> bool {
        (0..self.height).contains(&pos.row) && (0..self.width).contains(&pos.col)
    }

    fn at(&self, pos: &Position) -> Option<&Rc<RefCell<Tile>>> {
        if self.is_in(pos) {
            Some(&self.grid[pos.row][pos.col])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod exercise1 {
        use super::*;

        #[test]
        fn example_1() {
            let input = input::read_file("example_1.txt");
            let res = exercise1(&input);
            assert_eq!(res, 140);
        }

        #[test]
        fn example_2() {
            let input = input::read_file("example_2.txt");
            let res = exercise1(&input);
            assert_eq!(res, 772);
        }

        #[test]
        fn example_3() {
            let input = input::read_file("example_3.txt");
            let res = exercise1(&input);
            assert_eq!(res, 1930);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise1(&input);
            assert_eq!(res, 1371306);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn example2_1() {
            let input = input::read_file("example2_1.txt");
            let res = exercise2(&input);
            assert_eq!(res, 236);
        }

        #[test]
        fn example2_2() {
            let input = input::read_file("example2_2.txt");
            let res = exercise2(&input);
            assert_eq!(res, 368);
        }

        #[test]
        fn example_1() {
            let input = input::read_file("example_1.txt");
            let res = exercise2(&input);
            assert_eq!(res, 80);
        }

        #[test]
        fn example_2() {
            let input = input::read_file("example_2.txt");
            let res = exercise2(&input);
            assert_eq!(res, 436);
        }

        #[test]
        fn example_3() {
            let input = input::read_file("example_3.txt");
            let res = exercise2(&input);
            assert_eq!(res, 1206);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise2(&input);
            assert_eq!(res, 805880);
        }
    }
}
