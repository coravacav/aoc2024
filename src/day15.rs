use ahash::AHashSet;
use itertools::Itertools;

use crate::{
    Solution,
    direction::QuadDirection,
    grid::{Coord, Grid},
    panic_after,
};

pub struct Day15 {}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum MapMember {
    Empty,
    Robot,
    Wall,
    Crate,
    CrateLeft,
    CrateRight,
}

impl std::fmt::Display for MapMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Robot => write!(f, "@"),
            Self::Wall => write!(f, "#"),
            Self::Crate => write!(f, "O"),
            Self::CrateLeft => write!(f, "["),
            Self::CrateRight => write!(f, "]"),
        }
    }
}

impl Solution for Day15 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let (grid, dir_list) = input.split_once("\n\n").unwrap();

        let mut grid = Grid::new(grid, |c| match c {
            b'.' => MapMember::Empty,
            b'@' => MapMember::Robot,
            b'#' => MapMember::Wall,
            b'O' => MapMember::Crate,
            _ => unreachable!(),
        });

        let dir_list = dir_list
            .chars()
            .filter(|c| *c != '\n')
            .map(QuadDirection::from)
            .collect_vec();

        let mut robot_coord = grid
            .iter_with_coords()
            .find_map(|(c, m)| match m {
                MapMember::Robot => Some(c),
                _ => None,
            })
            .unwrap();

        // let mut lagging_direction = QuadDirection::None;
        // let mut before = vec![];
        // let mut after = vec![];

        for dir in dir_list {
            // for (line1, line2) in before.iter().zip(after.iter()) {
            //     println!("{line1} {lagging_direction} {line2}");
            // }
            // println!();

            // before = grid.pretty_print_into_rows();

            match grid[robot_coord + dir] {
                MapMember::Empty => {
                    grid[robot_coord] = MapMember::Empty;
                    robot_coord += dir;
                    grid[robot_coord] = MapMember::Robot;
                }
                MapMember::Wall => {}
                MapMember::Crate => {
                    let Some(mut coord_of_first_empty) = grid
                        .iter_direction_till(robot_coord, dir, |c| {
                            grid[c] == MapMember::Empty || grid[c] == MapMember::Wall
                        })
                        .find(|c| grid[*c] == MapMember::Empty || grid[*c] == MapMember::Wall)
                    else {
                        // println!("Can't move crate into wall");
                        // lagging_direction = dir;
                        // after = grid.pretty_print_into_rows();
                        continue;
                    };

                    if grid[coord_of_first_empty] == MapMember::Wall {
                        // println!("Can't move crate into wall");
                        // lagging_direction = dir;
                        // after = grid.pretty_print_into_rows();
                        continue;
                    }

                    let mut stop = false;

                    while !stop {
                        stop = grid[coord_of_first_empty - dir] == MapMember::Robot;

                        let tmp = grid[coord_of_first_empty];
                        grid[coord_of_first_empty] = grid[coord_of_first_empty - dir];
                        grid[coord_of_first_empty - dir] = tmp;

                        coord_of_first_empty -= dir;
                    }

                    robot_coord += dir;
                }
                _ => unreachable!(),
            }
            // lagging_direction = dir;
            // after = grid.pretty_print_into_rows();
        }

        grid.iter_with_coords()
            .filter_map(|(c, m)| match m {
                MapMember::Crate => Some(c.row() as usize * 100 + c.col() as usize),
                _ => None,
            })
            .sum::<usize>()
            .to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        Some(String::from("1568399"))
    }

    fn part2(&mut self, input: &str) -> String {
        let (grid, dir_list) = input.split_once("\n\n").unwrap();

        let small_grid = Grid::new(grid, |c| match c {
            b'.' => MapMember::Empty,
            b'@' => MapMember::Robot,
            b'#' => MapMember::Wall,
            b'O' => MapMember::Crate,
            _ => unreachable!(),
        });

        let mut grid = Grid::new_blank(small_grid.width * 2, small_grid.height, MapMember::Wall);

        for (coord, &v) in small_grid.iter_with_coords() {
            grid[Coord::new(coord.row(), coord.col() * 2)] = if v == MapMember::Crate {
                MapMember::CrateLeft
            } else {
                v
            };
            grid[Coord::new(coord.row(), coord.col() * 2 + 1)] = match v {
                MapMember::Crate => MapMember::CrateRight,
                MapMember::Robot => MapMember::Empty,
                _ => v,
            };
        }

        let dir_list = dir_list
            .chars()
            .filter(|c| *c != '\n')
            .map(QuadDirection::from)
            .collect_vec();

        let mut robot_coord = grid
            .iter_with_coords()
            .find_map(|(c, m)| match m {
                MapMember::Robot => Some(c),
                _ => None,
            })
            .unwrap();

        // let mut lagging_direction = QuadDirection::None;
        // let mut before: Vec<String> = vec![];
        // let mut after: Vec<String> = vec![];

        'next_dir: for dir in dir_list {
            // for (line1, line2) in before.iter().zip(after.iter()) {
            //     println!("{line1} {lagging_direction} {line2}");

            //     if line1.contains("[.")
            //         || line2.contains("[.")
            //         || line1.contains(".]")
            //         || line2.contains(".]")
            //     {
            //         panic!();
            //     }
            // }

            // println!();

            // before = grid.pretty_print_into_rows();

            match grid[robot_coord + dir] {
                MapMember::Empty => {
                    grid[robot_coord] = MapMember::Empty;
                    robot_coord += dir;
                    grid[robot_coord] = MapMember::Robot;
                }
                MapMember::Wall => {}
                MapMember::CrateLeft | MapMember::CrateRight => {
                    let mut search_stack = vec![robot_coord];
                    let mut historical_search_stack = AHashSet::new();
                    historical_search_stack.insert(robot_coord);

                    while let Some(coord) = search_stack.pop() {
                        match grid[coord + dir] {
                            MapMember::CrateLeft => {
                                if historical_search_stack
                                    .insert(coord + dir + QuadDirection::Right)
                                {
                                    search_stack.push(coord + dir + QuadDirection::Right);
                                }

                                if historical_search_stack.insert(coord + dir) {
                                    search_stack.push(coord + dir);
                                }
                            }
                            MapMember::CrateRight => {
                                if historical_search_stack.insert(coord + dir + QuadDirection::Left)
                                {
                                    search_stack.push(coord + dir + QuadDirection::Left);
                                }

                                if historical_search_stack.insert(coord + dir) {
                                    search_stack.push(coord + dir);
                                }
                            }
                            MapMember::Wall => {
                                // println!("Can't move crate into wall");
                                // lagging_direction = dir;
                                // after = grid.pretty_print_into_rows();
                                continue 'next_dir;
                            }
                            _ => {}
                        }
                    }

                    historical_search_stack
                        .into_iter()
                        .sorted_by(|a, b| match dir {
                            QuadDirection::Up => a.row().cmp(&b.row()),
                            QuadDirection::Down => b.row().cmp(&a.row()),
                            QuadDirection::Left => a.col().cmp(&b.col()),
                            QuadDirection::Right => b.col().cmp(&a.col()),
                            _ => unreachable!(),
                        })
                        .for_each(|c| {
                            let temp = grid[c];
                            grid[c] = grid[c + dir];
                            grid[c + dir] = temp;
                        });

                    robot_coord += dir;
                }
                _ => unreachable!(),
            }
            // lagging_direction = dir;
            // after = grid.pretty_print_into_rows();
        }

        // for (line1, line2) in before.iter().zip(after.iter()) {
        //     println!("{line1} {lagging_direction} {line2}");

        //     if line1.contains("[.")
        //         || line2.contains("[.")
        //         || line1.contains(".]")
        //         || line2.contains(".]")
        //     {
        //         panic!();
        //     }
        // }

        grid.iter_with_coords()
            .filter_map(|(c, m)| match m {
                MapMember::CrateLeft => Some(c.row() as usize * 100 + c.col() as usize),
                _ => None,
            })
            .sum::<usize>()
            .to_string()
    }

    fn known_solution_part2(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day15::new();

        assert_eq!(
            solution.part1(
                r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#
            ),
            String::from("2028")
        );

        assert_eq!(
            solution.part1(
                r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#
            ),
            String::from("10092")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day15::new();

        //         assert_eq!(
        //             solution.part2(
        //                 r#"#######
        // #...#.#
        // #.....#
        // #..OO@#
        // #..O..#
        // #.....#
        // #######

        // <vv<<^^<<^^"#
        //             ),
        //             String::from("105")
        //         );

        assert_eq!(
            solution.part2(
                r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#
            ),
            String::from("9021")
        );
    }
}
