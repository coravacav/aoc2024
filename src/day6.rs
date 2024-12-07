use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
    sync::Arc,
};

use ahash::AHashSet;
use dashmap::DashMap;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::Solution;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    row: isize,
    col: isize,
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Coord {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    fn new_usize(row: usize, col: usize) -> Self {
        Self::new(isize::try_from(row).unwrap(), isize::try_from(col).unwrap())
    }

    fn from_enumerated_grid<T>(grid: &Grid<T>, index: usize) -> Self {
        let i = isize::try_from(index).unwrap();

        Self::new(i / grid.width, i % grid.width)
    }

    fn in_bounds(&self, width: isize, height: isize) -> bool {
        self.row >= 0 && self.row < height && self.col >= 0 && self.col < width
    }
}

impl std::ops::Add<Coord> for Coord {
    type Output = Self;

    fn add(self, other: Coord) -> Self::Output {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl std::ops::AddAssign<Coord> for Coord {
    fn add_assign(&mut self, other: Coord) {
        self.row += other.row;
        self.col += other.col;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

impl Direction {
    fn from_direction(direction: char) -> Option<Self> {
        match direction {
            '^' => Some(Self::Up),
            'v' => Some(Self::Down),
            '>' => Some(Self::Right),
            '<' => Some(Self::Left),
            _ => None,
        }
    }

    fn rotate_right(self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    fn to_tuple_offset(self) -> Coord {
        match self {
            Self::Up => Coord { row: -1, col: 0 },
            Self::Down => Coord { row: 1, col: 0 },
            Self::Left => Coord { row: 0, col: -1 },
            Self::Right => Coord { row: 0, col: 1 },
        }
    }
}

#[derive(Debug, Clone)]
struct Grid<T> {
    data: Vec<T>,
    width: isize,
    height: isize,
}

#[derive(Debug)]
enum NextResult {
    Empty,
    HasBlock,
    OutOfBounds,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum GridType {
    Empty,
    Wall,
    Direction(Direction),
}

impl std::fmt::Display for GridType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridType::Empty => write!(f, "."),
            GridType::Wall => write!(f, "#"),
            GridType::Direction(dir) => write!(f, "{}", dir),
        }
    }
}

fn next_val(grid: &Grid<GridType>, coord: Coord, dir: Direction) -> NextResult {
    let next_coord = coord + dir.to_tuple_offset();

    if !next_coord.in_bounds(grid.width, grid.height) {
        return NextResult::OutOfBounds;
    }

    match grid[next_coord] {
        GridType::Wall => NextResult::HasBlock,
        _ => NextResult::Empty,
    }
}

impl<T> Grid<T> {
    fn new(input: &str, char_to_t: impl Fn(char) -> T) -> Self {
        let mut grid = vec![];
        let mut width = None;
        let mut height = 0;

        for line in input.lines() {
            let line = line.chars().map(&char_to_t).collect_vec();

            if width.is_none() {
                width = Some(isize::try_from(line.len()).unwrap());
            }

            height += 1;

            grid.extend(line);
        }

        Self {
            width: width.unwrap(),
            height,
            data: grid,
        }
    }

    fn iter_with_coords(&self) -> impl DoubleEndedIterator<Item = (Coord, &T)> {
        self.data.iter().enumerate().map(|(i, t)| {
            let i = isize::try_from(i).unwrap();

            (Coord::new(i / self.width, i % self.width), t)
        })
    }

    fn iter_lines(&self) -> impl DoubleEndedIterator<Item = &[T]> {
        self.data.chunks(self.width as usize)
    }
}

impl<T: Display> Grid<T> {
    fn pretty_print(&self) {
        for line in self.iter_lines() {
            println!("{}", line.iter().map(|t| t.to_string()).join(""));
        }
    }

    fn pretty_print_bolded_coord(&self, coord: Coord) {
        for (i, line) in self.iter_lines().enumerate() {
            println!(
                "{}",
                line.iter()
                    .enumerate()
                    .map(|(j, t)| if Coord::new_usize(i, j) == coord {
                        use colored::*;
                        format!("{}", t).on_bright_red().black().to_string()
                    } else {
                        t.to_string()
                    })
                    .join("")
            );
        }
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, coord: Coord) -> &Self::Output {
        &self.data[coord.col as usize + coord.row as usize * self.width as usize]
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.data[index.col as usize + index.row as usize * self.width as usize]
    }
}

fn get_visited_cells_till_exit(
    grid: &Grid<GridType>,
    mut coord: Coord,
    mut dir: Direction,
) -> AHashSet<Coord> {
    let mut visited_cells = AHashSet::new();

    loop {
        visited_cells.insert(coord);

        match next_val(grid, coord, dir) {
            NextResult::HasBlock => {
                dir = dir.rotate_right();
            }
            NextResult::Empty => {
                coord += dir.to_tuple_offset();
            }
            NextResult::OutOfBounds => {
                break;
            }
        }
    }

    visited_cells
}

pub struct Day6 {}

impl Solution for Day6 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let grid = Grid::new(input, |c| match c {
            '.' => GridType::Empty,
            '#' => GridType::Wall,
            '^' => GridType::Direction(Direction::Up),
            'v' => GridType::Direction(Direction::Down),
            '>' => GridType::Direction(Direction::Right),
            '<' => GridType::Direction(Direction::Left),
            _ => unreachable!(),
        });

        // find the first direction
        let (coord, dir) = grid
            .iter_with_coords()
            .find_map(|(coord, val)| match val {
                GridType::Direction(dir) => Some((coord, dir)),
                _ => None,
            })
            .unwrap();

        let visited_cells = get_visited_cells_till_exit(&grid, coord, *dir);

        visited_cells.len().to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        Some(String::from("5131"))
    }

    fn part2(&mut self, input: &str) -> String {
        let grid = Grid::new(input, |c| match c {
            '.' => GridType::Empty,
            '#' => GridType::Wall,
            '^' => GridType::Direction(Direction::Up),
            'v' => GridType::Direction(Direction::Down),
            '>' => GridType::Direction(Direction::Right),
            '<' => GridType::Direction(Direction::Left),
            _ => unreachable!(),
        });

        // find the first direction
        let (starting_coord, starting_dir) = grid
            .iter_with_coords()
            .find_map(|(coord, val)| match val {
                GridType::Direction(dir) => Some((coord, dir)),
                _ => None,
            })
            .unwrap();

        let visited_cells = get_visited_cells_till_exit(&grid, starting_coord, *starting_dir);

        visited_cells
            .par_iter()
            .map(|to_edit_coord| {
                let mut coord = starting_coord;
                let mut dir = *starting_dir;
                let mut visited_cells = AHashSet::new();

                visited_cells.insert((coord, dir));

                loop {
                    let next = if *to_edit_coord == (coord + dir.to_tuple_offset()) {
                        NextResult::HasBlock
                    } else {
                        next_val(&grid, coord, dir)
                    };

                    match next {
                        NextResult::HasBlock => {
                            dir = dir.rotate_right();
                        }
                        NextResult::Empty => {
                            coord += dir.to_tuple_offset();
                            if !visited_cells.insert((coord, dir)) {
                                return 1;
                            }
                        }
                        NextResult::OutOfBounds => {
                            return 0;
                        }
                    }
                }
            })
            .sum::<u16>()
            .to_string()
    }

    fn known_solution_part2(&self) -> Option<String> {
        Some("1784".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day6::new();
        assert_eq!(
            solution.part1(
                r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#
            ),
            String::from("41")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day6::new();
        assert_eq!(
            solution.part2(
                r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#
            ),
            String::from("6")
        );
    }
}
