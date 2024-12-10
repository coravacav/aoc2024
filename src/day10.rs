use ahash::AHashSet;

use crate::{
    Solution,
    direction::QuadDirection,
    grid::{Coord, Grid},
};

pub struct Day10 {}

impl Solution for Day10 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let grid = Grid::new(input, |c| c);

        grid.iter_with_coords()
            .filter(|(_, c)| **c == b'0')
            .map(|(c, _)| {
                #[derive(Debug, Copy, Clone)]
                struct Item(Coord, QuadDirection);
                impl std::hash::Hash for Item {
                    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                        self.0.hash(state);
                    }
                }
                impl PartialEq for Item {
                    fn eq(&self, other: &Self) -> bool {
                        self.0 == other.0
                    }
                }
                impl Eq for Item {}

                let mut current_stack: AHashSet<Item> = AHashSet::new();
                current_stack.insert(Item(c, QuadDirection::None));

                for next in b'1'..=b'9' {
                    let mut next_stack = AHashSet::new();
                    for Item(coord, direction) in current_stack {
                        for &dir in direction.get_non_opposite_directions() {
                            let next_coord = coord + dir;
                            if let Some(next_value) = grid.get(next_coord) {
                                if *next_value == next {
                                    next_stack.insert(Item(next_coord, dir));
                                }
                            }
                        }
                    }
                    current_stack = next_stack;

                    if current_stack.is_empty() {
                        return 0;
                    }
                }

                current_stack.len()
            })
            .sum::<usize>()
            .to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        Some(String::from("674"))
    }

    fn part2(&mut self, input: &str) -> String {
        let grid = Grid::new(input, |c| c);

        grid.iter_with_coords()
            .filter(|(_, c)| **c == b'0')
            .map(|(c, _)| {
                let mut current_stack: Vec<(Coord, QuadDirection)> = vec![(c, QuadDirection::None)];

                for next in b'1'..=b'9' {
                    let mut next_stack = Vec::new();
                    for (coord, from_direction) in current_stack {
                        for &to_direction in from_direction.get_non_opposite_directions() {
                            let next_coord = coord + to_direction;
                            if let Some(next_value) = grid.get(next_coord) {
                                if *next_value == next {
                                    next_stack.push((next_coord, to_direction));
                                }
                            }
                        }
                    }
                    current_stack = next_stack;

                    if current_stack.is_empty() {
                        return 0;
                    }
                }

                current_stack.len()
            })
            .sum::<usize>()
            .to_string()
    }

    fn known_solution_part2(&self) -> Option<String> {
        Some(String::from("1372"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day10::new();
        assert_eq!(
            solution.part1(
                r#"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"#
            ),
            String::from("2")
        );

        assert_eq!(
            solution.part1(
                r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#
            ),
            String::from("4")
        );

        assert_eq!(
            solution.part1(
                r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#
            ),
            String::from("36")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day10::new();
        assert_eq!(
            solution.part2(
                r#".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9...."#
            ),
            String::from("3")
        );

        assert_eq!(
            solution.part2(
                r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#
            ),
            String::from("13")
        );

        assert_eq!(
            solution.part2(
                r#"012345
123456
234567
345678
4.6789
56789."#
            ),
            String::from("227")
        );

        assert_eq!(
            solution.part2(
                r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#
            ),
            String::from("81")
        );
    }
}
