use crate::grid::position::Position;

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(vec2d: Vec<Vec<T>>) -> Self {
        Self {
            width: vec2d.first().expect("Empty vector").len(),
            height: vec2d.len(),
            data: vec2d,
        }
    }

    pub fn get(&self, pos: &Position) -> Option<&T> {
        Some(self.data.get(pos.row)?.get(pos.col)?)
    }

    pub fn get_mut(&mut self, pos: &Position) -> Option<&mut T> {
        Some(self.data.get_mut(pos.row)?.get_mut(pos.col)?)
    }

    pub fn put(&mut self, pos: &Position, to_put: T) -> bool {
        if let Some(entry) = self.get_mut(pos) {
            *entry = to_put;
            true
        } else {
            false
        }
    }

    /// Returns an iterator yielding tuples (position, reference to value) for
    /// each element in the grid.
    ///
    /// Iteration proceeds row by row, column by column within each row.
    pub fn iter(&self) -> impl Iterator<Item = (Position, &T)> + '_ {
        self.data.iter().enumerate().flat_map(|(row, row_vec)| {
            row_vec
                .iter()
                .enumerate()
                .map(move |(col, value)| (Position::new(row, col), value))
        })
    }

    /// Returns an iterator yielding tuples (position, mutable reference to value)
    /// for each element in the grid.
    ///
    /// Iteration proceeds row by row, column by column within each row.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Position, &mut T)> + '_ {
        self.data.iter_mut().enumerate().flat_map(|(row, row_vec)| {
            row_vec
                .iter_mut()
                .enumerate()
                .map(move |(col, value)| (Position::new(row, col), value))
        })
    }
}

impl From<&str> for Grid<char> {
    fn from(value: &str) -> Self {
        Self::new(value.lines().map(|line| line.chars().collect()).collect())
    }
}
