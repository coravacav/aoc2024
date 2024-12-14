use itertools::Itertools;

use crate::{
    Solution,
    direction::QuadDirection,
    grid::{Coord, Grid},
};

pub struct Day14 {
    wide: i64,
    tall: i64,
}

impl Day14 {
    fn new_with_size(wide: i64, tall: i64) -> Self {
        Self { wide, tall }
    }
}

impl Solution for Day14 {
    fn new() -> Self {
        Self {
            wide: 101,
            tall: 103,
        }
    }

    fn part1(&mut self, input: &str) -> String {
        let mut top_left: usize = 0;
        let mut top_right: usize = 0;
        let mut bottom_left: usize = 0;
        let mut bottom_right: usize = 0;

        for line in input.lines() {
            let (p, v) = line.split_once(" ").unwrap();
            let p = p.split_once("=").unwrap().1;
            let v = v.split_once("=").unwrap().1;
            let (p1, p2) = p.split_once(",").unwrap();
            let (v1, v2) = v.split_once(",").unwrap();
            let p1 = p1.parse::<i64>().unwrap();
            let p2 = p2.parse::<i64>().unwrap();
            let v1 = v1.parse::<i64>().unwrap();
            let v2 = v2.parse::<i64>().unwrap();

            let col = (p1 + v1 * 100).rem_euclid(self.wide);
            let row = (p2 + v2 * 100).rem_euclid(self.tall);

            if col < self.wide / 2 && row < self.tall / 2 {
                top_left += 1;
            } else if col > self.wide / 2 && row < self.tall / 2 {
                top_right += 1;
            } else if col < self.wide / 2 && row > self.tall / 2 {
                bottom_left += 1;
            } else if col > self.wide / 2 && row > self.tall / 2 {
                bottom_right += 1;
            }
        }

        (top_left * top_right * bottom_left * bottom_right).to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        None
    }

    fn part2(&mut self, input: &str) -> String {
        let mut starting_coords = vec![];
        let mut velocities = vec![];

        for line in input.lines() {
            let (p, v) = line.split_once(" ").unwrap();
            let p = p.split_once("=").unwrap().1;
            let v = v.split_once("=").unwrap().1;
            let (p1, p2) = p.split_once(",").unwrap();
            let (v1, v2) = v.split_once(",").unwrap();
            let p1 = p1.parse::<i64>().unwrap();
            let p2 = p2.parse::<i64>().unwrap();
            let v1 = v1.parse::<i64>().unwrap();
            let v2 = v2.parse::<i64>().unwrap();

            starting_coords.push((p1, p2));
            velocities.push((v1, v2));
        }

        // wait for user to press enter
        for i in 1.. {
            let coords = starting_coords
                .iter()
                .zip(velocities.iter())
                .map(|(&(p1, p2), &(v1, v2))| {
                    Coord::new(
                        (p1 + v1 * i).rem_euclid(self.wide) as i16,
                        (p2 + v2 * i).rem_euclid(self.tall) as i16,
                    )
                })
                .collect_vec();

            #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
            struct Edge {
                from: Coord,
                to: Coord,
            }

            impl Edge {
                fn combine_edges(self, other: Self) -> Option<Self> {
                    if self.from.is_adjacent(other.to) || self.to.is_adjacent(other.from) {
                        let (from, to) = [(other.from, self.to), (other.to, self.from)]
                            .iter()
                            .copied()
                            .max_by_key(|&(x, y)| x.manhattan_distance(y))
                            .unwrap();

                        return Some(Edge { from, to });
                    }

                    None
                }
            }

            let edges: Vec<Edge> = coords
                .iter()
                .map(|&coord| Edge {
                    from: coord,
                    to: coord,
                })
                .fold(vec![], |mut all: Vec<Edge>, edge| {
                    let mut combined = false;
                    for saved_edge in all.iter_mut() {
                        if let Some(new_edge) = saved_edge.combine_edges(edge) {
                            *saved_edge = new_edge;
                            combined = true;
                        }
                    }
                    if !combined {
                        all.push(edge);
                    }

                    all
                });

            if !edges
                .iter()
                .any(|edge| edge.from.manhattan_distance(edge.to) > 5)
            {
                continue;
            }

            let mut grid = Grid::new_blank(self.wide as i16, self.tall as i16, ' ');
            grid.set_all_coords_to(coords.iter().copied(), '.');
            grid.pretty_print();
        }

        String::new()
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
        let mut solution = Day14::new_with_size(11, 7);
        assert_eq!(
            solution.part1(
                r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#
            ),
            String::from("12")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day14::new();
        assert_eq!(solution.part2(r#""#), String::from(""));
    }
}
