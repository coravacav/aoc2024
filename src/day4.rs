use itertools::Itertools;

use crate::Solution;

pub struct Day4 {}

#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<u8>>,
    col_count: i16,
    row_count: i16,
}

impl Matrix {
    fn new(input: &str) -> Self {
        let mut input_array: Vec<Vec<u8>> = vec![];
        let col_count = input.lines().next().unwrap().chars().count();
        let row_count = input.lines().count();

        for line in input.lines() {
            let mut line_array = vec![];
            for c in line.as_bytes() {
                line_array.push(*c);
            }
            input_array.push(line_array);
        }

        Self {
            data: input_array,
            col_count: col_count as i16,
            row_count: row_count as i16,
        }
    }

    fn get(&self, row: i16, col: i16, inc_row: i16, inc_col: i16) -> Option<u8> {
        let (row, col) = (row + inc_row, col + inc_col);
        if row < 0 || col < 0 || col >= self.col_count || row >= self.row_count {
            None
        } else {
            Some(self.data[row as usize][col as usize])
        }
    }

    fn iterate_all(&self) -> impl Iterator<Item = (u8, u8, u8, u8)> {
        (-1..=1).cartesian_product(-1..=1).flat_map(move |(x, y)| {
            (0..self.row_count)
                .flat_map(move |row| (0..self.col_count).map(move |col| (row, col)))
                .flat_map(move |(row, col)| {
                    if let (Some(a), Some(b), Some(c), Some(d)) = (
                        self.get(row, col, 0, 0),
                        self.get(row, col, x, y),
                        self.get(row, col, x * 2, y * 2),
                        self.get(row, col, x * 3, y * 3),
                    ) {
                        Some((a, b, c, d))
                    } else {
                        None
                    }
                })
        })
    }

    fn iterate_all_2(&self) -> impl Iterator<Item = (u8, u8, u8, u8, u8)> {
        [
            [(-1, -1), (-1, 1), (1, -1), (1, 1)],
            [(-1, 1), (1, 1), (-1, -1), (1, -1)],
            [(-1, 1), (-1, -1), (1, 1), (1, -1)],
            [(1, 1), (-1, -1), (1, -1), (-1, 1)],
        ]
        .iter()
        .flat_map(move |xys| {
            (0..self.row_count)
                .flat_map(move |row| (0..self.col_count).map(move |col| (row, col)))
                .flat_map(move |(row, col)| {
                    if let (Some(a), Some(b), Some(c), Some(d), Some(e)) = (
                        self.get(row, col, xys[0].0, xys[0].1),
                        self.get(row, col, xys[1].0, xys[1].1),
                        self.get(row, col, 0, 0),
                        self.get(row, col, xys[2].0, xys[2].1),
                        self.get(row, col, xys[3].0, xys[3].1),
                    ) {
                        Some((a, b, c, d, e))
                    } else {
                        None
                    }
                })
        })
    }
}

impl Solution for Day4 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        Matrix::new(input)
            .iterate_all()
            .filter(|c| matches!(c, (b'X', b'M', b'A', b'S')))
            .count()
            .to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        Some("2618".to_string())
    }

    fn part2(&mut self, input: &str) -> String {
        Matrix::new(input)
            .iterate_all_2()
            .filter(|c| matches!(c, (b'M', b'S', b'A', b'M', b'S')))
            .count()
            .to_string()
    }

    fn known_solution_part2(&self) -> Option<String> {
        Some("2011".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day4::new();

        assert_eq!(
            solution.part1(
                r#"....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX"#
            ),
            String::from("18")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day4::new();

        assert_eq!(
            solution.part2(
                r#".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."#
            ),
            String::from("9")
        );
    }
}
