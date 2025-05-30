#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Checks if a position is within the rectangle defined by (0, 0) and self
    /// (inclusive).
    pub fn contains(&self, pos: &Self) -> bool {
        pos.row <= self.row && pos.col <= self.col
    }

    /// Returns an iterator over adjacent positions in clockwise order
    /// (Up, Right, Down, Left), skipping positions with negative row or column
    /// values.
    pub fn neighbors(&self) -> impl Iterator<Item = Self> {
        [
            (self.row > 0).then(|| Self::new(self.row - 1, self.col)),
            Some(Self::new(self.row, self.col + 1)),
            Some(Self::new(self.row + 1, self.col)),
            (self.col > 0).then(|| Self::new(self.row, self.col - 1)),
        ]
        .into_iter()
        .flatten()
    }

    /// Returns an iterator over adjacent positions in clockwise order
    /// (Up, Right, Down, Left), skipping positions outside the rectangle
    /// defined by (0,0) and the given max position.
    pub fn neighbors_contained(&self, max: Self) -> impl Iterator<Item = Self> {
        [
            (self.row > 0).then(|| Self::new(self.row - 1, self.col)),
            Some(Self::new(self.row, self.col + 1)),
            Some(Self::new(self.row + 1, self.col)),
            (self.col > 0).then(|| Self::new(self.row, self.col - 1)),
        ]
        .into_iter()
        .flatten()
        .filter(move |pos| max.contains(pos))
    }

    // fn to(&self, direction: Direction) -> Self {
    //     match direction {
    //         Direction::Up => Self::new(self.row - 1, self.col),
    //         Direction::Down => Self::new(self.row + 1, self.col),
    //         Direction::Right => Self::new(self.row, self.col + 1),
    //         Direction::Left => Self::new(self.row, self.col - 1),
    //     }
    // }

    // fn dir(&self, from: Position) -> Direction {
    //     let row_diff = if self.row >= from.row {
    //         self.row - from.row
    //     } else {
    //         from.row - self.row
    //     };
    //     let col_diff = if self.col >= from.col {
    //         self.col - from.col
    //     } else {
    //         from.col - self.col
    //     };

    //     if col_diff >= row_diff {
    //         if self.col >= from.col {
    //             Direction::Right
    //         } else {
    //             Direction::Left
    //         }
    //     } else {
    //         if self.row <= from.row {
    //             Direction::Up
    //         } else {
    //             Direction::Down
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(Position::new(5, 10), Position { row: 5, col: 10 });
    }

    #[test]
    fn contains() {
        let bounds = Position::new(5, 5);
        assert!(bounds.contains(&Position::new(0, 0)));
        assert!(bounds.contains(&Position::new(5, 5)));
        assert!(bounds.contains(&Position::new(3, 4)));
        assert!(!bounds.contains(&Position::new(6, 5)));
        assert!(!bounds.contains(&Position::new(5, 6)));
        assert!(!bounds.contains(&Position::new(6, 6)));
    }

    #[test]
    fn neighbors_middle() {
        let pos = Position::new(2, 2);
        let neighbors: Vec<_> = pos.neighbors().collect();
        let expected = vec![
            Position::new(1, 2), // Up
            Position::new(2, 3), // Right
            Position::new(3, 2), // Down
            Position::new(2, 1), // Left
        ];
        assert_eq!(neighbors, expected);
    }

    #[test]
    fn neighbors_origin() {
        let pos = Position::new(0, 0);
        let neighbors: Vec<_> = pos.neighbors().collect();
        let expected = vec![
            Position::new(0, 1), // Right
            Position::new(1, 0), // Down
        ];
        assert_eq!(neighbors, expected);
    }

    #[test]
    fn neighbors_edge() {
        let pos = Position::new(0, 2);
        let neighbors: Vec<_> = pos.neighbors().collect();
        let expected = vec![
            Position::new(0, 3), // Right
            Position::new(1, 2), // Down
            Position::new(0, 1), // Left
        ];
        assert_eq!(neighbors, expected);
    }

    #[test]
    fn neighbor_contained_middle() {
        let pos = Position::new(1, 1);
        let max = Position::new(2, 2);
        let neighbors: Vec<_> = pos.neighbors_contained(max).collect();
        let expected = vec![
            Position::new(0, 1), // Up
            Position::new(1, 2), // Right
            Position::new(2, 1), // Down
            Position::new(1, 0), // Left
        ];
        assert_eq!(neighbors, expected);
    }

    #[test]
    fn neighbor_contained_corner() {
        let pos = Position::new(2, 2);
        let max = Position::new(2, 2); // Max is the position itself
        let neighbors: Vec<_> = pos.neighbors_contained(max).collect();
        let expected = vec![
            Position::new(1, 2), // Up
            // Right (2, 3) is out of bounds
            // Down (3, 2) is out of bounds
            Position::new(2, 1), // Left
        ];
        assert_eq!(neighbors, expected);
    }

    #[test]
    fn neighbor_contained_origin_bounded() {
        let pos = Position::new(0, 0);
        let max = Position::new(0, 0); // Max is the origin
        let neighbors: Vec<_> = pos.neighbors_contained(max).collect();
        let expected = vec![
            // Right (0, 1) is out of bounds
            // Down (1, 0) is out of bounds
        ];
        assert_eq!(neighbors, expected);
    }
}
