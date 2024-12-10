use std::fmt::Debug;

use ahash::AHashSet;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    Solution,
    direction::QuadDirection,
    grid::{Coord, Grid},
};

#[derive(Debug, Clone, Copy)]
enum GridType {
    Empty,
    Wall,
    Direction(QuadDirection),
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

#[derive(Debug)]
enum NextResult {
    Empty,
    HasBlock,
    OutOfBounds,
}

fn next_val(grid: &Grid<GridType>, coord: Coord, dir: QuadDirection) -> NextResult {
    let next_coord = coord + dir.to_coord_offset();

    if !next_coord.in_bounds(grid.width, grid.height) {
        return NextResult::OutOfBounds;
    }

    match grid[next_coord] {
        GridType::Wall => NextResult::HasBlock,
        _ => NextResult::Empty,
    }
}

fn get_visited_cells_till_exit(
    grid: &Grid<GridType>,
    mut coord: Coord,
    mut dir: QuadDirection,
) -> AHashSet<Coord> {
    let mut visited_cells = AHashSet::new();

    loop {
        visited_cells.insert(coord);

        match next_val(grid, coord, dir) {
            NextResult::HasBlock => {
                dir = dir.rotate_right();
            }
            NextResult::Empty => {
                coord += dir.to_coord_offset();
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
            b'.' => GridType::Empty,
            b'#' => GridType::Wall,
            b'^' => GridType::Direction(QuadDirection::Up),
            b'v' => GridType::Direction(QuadDirection::Down),
            b'>' => GridType::Direction(QuadDirection::Right),
            b'<' => GridType::Direction(QuadDirection::Left),
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
            b'.' => GridType::Empty,
            b'#' => GridType::Wall,
            b'^' => GridType::Direction(QuadDirection::Up),
            b'v' => GridType::Direction(QuadDirection::Down),
            b'>' => GridType::Direction(QuadDirection::Right),
            b'<' => GridType::Direction(QuadDirection::Left),
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
                    let next = if *to_edit_coord == (coord + dir.to_coord_offset()) {
                        NextResult::HasBlock
                    } else {
                        next_val(&grid, coord, dir)
                    };

                    match next {
                        NextResult::HasBlock => {
                            dir = dir.rotate_right();
                        }
                        NextResult::Empty => {
                            coord += dir.to_coord_offset();
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
