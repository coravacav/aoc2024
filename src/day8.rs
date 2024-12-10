use itertools::{Itertools, chain};

use crate::{Solution, grid::Grid};

pub struct Day8 {}

impl Solution for Day8 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let grid = Grid::new(input, |c| c);

        let binding = grid
            .iter_with_coords()
            .filter(|(_, c)| **c != b'.')
            .into_group_map_by(|(_, c)| **c);

        let non_empty_grid_locations = binding
            .into_iter()
            .flat_map(|(_, v)| {
                v.into_iter()
                    .map(|(coord, _)| coord)
                    .combinations(2)
                    .flat_map(|v| {
                        let c1 = v[0];
                        let c2 = v[1];

                        let difference = c1 - c2;
                        [c1 + difference, c2 - difference]
                    })
            })
            .filter(|c| grid.is_coord_in_bounds(*c))
            .unique();

        non_empty_grid_locations.count().to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        Some(String::from("409"))
    }

    fn part2(&mut self, input: &str) -> String {
        let grid = Grid::new(input, |c| c);

        let binding = grid
            .iter_with_coords()
            .filter(|(_, c)| **c != b'.')
            .into_group_map_by(|(_, c)| **c);

        let non_empty_grid_locations = binding
            .into_iter()
            .flat_map(|(_, v)| {
                v.into_iter()
                    .map(|(coord, _)| coord)
                    .combinations(2)
                    .flat_map(|v| {
                        let c1 = v[0];
                        let c2 = v[1];

                        let difference = c1 - c2;

                        chain!(
                            (0..)
                                .map(move |i| c1 + difference * i)
                                .take_while(|c| grid.is_coord_in_bounds(*c)),
                            (0..)
                                .map(move |i| c2 - difference * i)
                                .take_while(|c| grid.is_coord_in_bounds(*c))
                        )
                    })
            })
            .unique();

        non_empty_grid_locations.count().to_string()
    }

    fn known_solution_part2(&self) -> Option<String> {
        Some(String::from("1308"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day8::new();
        assert_eq!(
            solution.part1(
                r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#
            ),
            String::from("14")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day8::new();
        assert_eq!(
            solution.part2(
                r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#
            ),
            String::from("34")
        );
    }
}
