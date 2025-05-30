// #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// enum Direction {
//     Up,
//     Down,
//     Right,
//     Left,
// }

// impl Direction {
//     fn clockwise(&self) -> Self {
//         match self {
//             Direction::Up => Direction::Right,
//             Direction::Right => Direction::Down,
//             Direction::Down => Direction::Left,
//             Direction::Left => Direction::Up,
//         }
//     }

//     fn counter_clockwise(&self) -> Self {
//         match self {
//             Direction::Up => Direction::Left,
//             Direction::Left => Direction::Down,
//             Direction::Down => Direction::Right,
//             Direction::Right => Direction::Up,
//         }
//     }
// }

// impl std::fmt::Display for Direction {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 Direction::Up => '^',
//                 Direction::Down => 'v',
//                 Direction::Right => '>',
//                 Direction::Left => '<',
//             }
//         )
//     }
// }
