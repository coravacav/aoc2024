use itertools::Itertools;

use crate::direction::QuadDirection;

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Coord {
    row: i16,
    col: i16,
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Coord {
    pub fn new(row: i16, col: i16) -> Self {
        Self { row, col }
    }

    pub fn new_usize(row: usize, col: usize) -> Self {
        Self::new(i16::try_from(row).unwrap(), i16::try_from(col).unwrap())
    }

    #[allow(dead_code)]
    pub fn from_enumerated_grid<T>(grid: &Grid<T>, index: usize) -> Self {
        let i = i16::try_from(index).unwrap();

        Self::new(i / grid.width, i % grid.width)
    }

    pub fn in_bounds(&self, width: i16, height: i16) -> bool {
        self.row >= 0 && self.row < height && self.col >= 0 && self.col < width
    }

    pub fn row(&self) -> i16 {
        self.row
    }

    pub fn col(&self) -> i16 {
        self.col
    }

    #[allow(clippy::nonminimal_bool)]
    pub fn is_adjacent(&self, other: Self) -> bool {
        (self.row - 1 == other.row && self.col == other.col)
            || (self.row + 1 == other.row && self.col == other.col)
            || (self.col - 1 == other.col && self.row == other.row)
            || (self.col + 1 == other.col && self.row == other.row)
    }

    pub fn manhattan_distance(&self, other: Self) -> i16 {
        (self.row - other.row).abs() + (self.col - other.col).abs()
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

impl std::ops::Add<i16> for Coord {
    type Output = Self;

    fn add(self, rhs: i16) -> Self::Output {
        Self {
            row: self.row + rhs,
            col: self.col + rhs,
        }
    }
}

impl std::ops::Sub<Coord> for Coord {
    type Output = Self;

    fn sub(self, other: Coord) -> Self::Output {
        Self {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

impl std::ops::SubAssign<Coord> for Coord {
    fn sub_assign(&mut self, other: Coord) {
        self.row -= other.row;
        self.col -= other.col;
    }
}

impl std::ops::Mul<i16> for Coord {
    type Output = Self;

    fn mul(self, other: i16) -> Self::Output {
        Self {
            row: self.row * other,
            col: self.col * other,
        }
    }
}

impl std::ops::AddAssign<Coord> for Coord {
    fn add_assign(&mut self, other: Coord) {
        self.row += other.row;
        self.col += other.col;
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    pub width: i16,
    pub height: i16,
}

impl<T> Grid<T> {
    pub fn new(input: &str, char_to_t: impl Fn(u8) -> T) -> Self {
        let mut grid = Vec::with_capacity(input.len());
        let mut height = 0;

        for c in input.as_bytes() {
            match c {
                b'\n' => {
                    height += 1;
                }
                c => {
                    grid.push(char_to_t(*c));
                }
            }
        }

        let height = if input.as_bytes().last() != Some(&b'\n') {
            height + 1
        } else {
            height
        };

        // Check if the width = height length makes sense
        let usize_height = height;
        let guess_size = usize_height * usize_height + usize_height - 1;

        let width = if input.len() != guess_size {
            // First length till newline
            let mut width = 0;
            for c in input.as_bytes() {
                if *c == b'\n' {
                    break;
                }
                width += 1;
            }

            width
        } else {
            // assert that it's square
            assert_eq!(input.len(), height * (height + 1) - 1);

            height
        };

        Self {
            width: width as i16,
            height: height as i16,
            data: grid,
        }
    }

    pub fn new_blank(width: i16, height: i16, value: T) -> Self
    where
        T: Clone,
    {
        Self {
            width,
            height,
            data: vec![value; width as usize * height as usize],
        }
    }

    pub fn set_all_coords_to(&mut self, coords: impl Iterator<Item = Coord>, value: T)
    where
        T: Clone,
    {
        for coord in coords {
            self[coord] = value.clone();
        }
    }

    pub fn iter_with_coords(&self) -> impl DoubleEndedIterator<Item = (Coord, &T)> {
        self.data.iter().enumerate().map(|(i, t)| {
            let i = i16::try_from(i).unwrap();

            (Coord::new(i / self.width, i % self.width), t)
        })
    }

    pub fn iter_lines(&self) -> impl DoubleEndedIterator<Item = &[T]> {
        self.data.chunks(self.width as usize)
    }

    pub fn iter_direction_till(
        &self,
        coord: Coord,
        dir: QuadDirection,
        stop_check: impl Fn(Coord) -> bool,
    ) -> impl Iterator<Item = Coord> {
        let mut coord = coord;
        let mut stop = false;

        std::iter::from_fn(move || {
            if stop {
                return None;
            }

            coord += dir;

            if !self.is_coord_in_bounds(coord) || stop_check(coord) {
                stop = true;
                return Some(coord);
            }

            Some(coord)
        })
    }

    pub fn is_coord_in_bounds(&self, coord: Coord) -> bool {
        coord.in_bounds(self.width, self.height)
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        if !self.is_coord_in_bounds(coord) {
            return None;
        }

        Some(&self[coord])
    }
}

impl<T: std::fmt::Display> Grid<T> {
    pub fn pretty_print(&self) {
        for line in self.iter_lines() {
            println!("{}", line.iter().map(|t| t.to_string()).join(""));
        }
    }

    pub fn pretty_print_into_rows(&self) -> Vec<String> {
        self.iter_lines()
            .map(|line| line.iter().map(|t| t.to_string()).join(""))
            .collect()
    }

    #[allow(dead_code)]
    pub fn pretty_print_bolded_coord(&self, coord: Coord) {
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

    pub fn pretty_print_bolded_coords(&self, coords: &[Coord]) {
        for (i, line) in self.iter_lines().enumerate() {
            println!(
                "{}",
                line.iter()
                    .enumerate()
                    .map(|(j, t)| if coords.contains(&Coord::new_usize(i, j)) {
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

impl<T> std::ops::Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, coord: Coord) -> &Self::Output {
        &self.data[coord.col as usize + coord.row as usize * self.width as usize]
    }
}

impl<T> std::ops::IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.data[index.col as usize + index.row as usize * self.width as usize]
    }
}
