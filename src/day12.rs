use ahash::AHashSet;

use crate::{
    Solution,
    direction::QuadDirection,
    grid::{Coord, Grid},
};

pub struct Day12 {}

impl Solution for Day12 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let grid = Grid::new(input, |c| c as char);
        let mut visited = Grid::new(input, |_| false);
        let mut perimeter_sum = 0;

        for (coord, &value) in grid.iter_with_coords() {
            if visited[coord] {
                continue;
            }

            let mut stack = vec![coord];
            let mut output_stack = AHashSet::new();
            output_stack.insert(coord);

            while let Some(current) = stack.pop() {
                if visited[current] {
                    continue;
                }
                visited[current] = true;

                for &direction in QuadDirection::get_all_directions() {
                    let neighbor = current + direction;
                    if grid.is_coord_in_bounds(neighbor)
                        && !visited[neighbor]
                        && grid[neighbor] == value
                    {
                        stack.push(neighbor);
                        output_stack.insert(neighbor);
                    }
                }
            }

            let mut region_perimeter = 0;
            for &current in &output_stack {
                for &direction in QuadDirection::get_all_directions() {
                    let neighbor = current + direction;
                    if !grid.is_coord_in_bounds(neighbor) || grid[neighbor] != value {
                        region_perimeter += 1;
                    }
                }
            }

            perimeter_sum += region_perimeter * output_stack.len();
        }

        perimeter_sum.to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        Some(String::from("1461752"))
    }

    fn part2(&mut self, input: &str) -> String {
        let grid = Grid::new(input, |c| c as char);
        let mut visited = Grid::new(input, |_| false);
        let mut perimeter_sum = 0;

        for (coord, &value) in grid.iter_with_coords() {
            if visited[coord] {
                continue;
            }

            let mut stack = vec![coord];
            let mut output_stack = AHashSet::new();
            output_stack.insert(coord);

            while let Some(current) = stack.pop() {
                if visited[current] {
                    continue;
                }
                visited[current] = true;

                for &direction in QuadDirection::get_all_directions() {
                    let neighbor = current + direction;
                    if grid.is_coord_in_bounds(neighbor)
                        && !visited[neighbor]
                        && grid[neighbor] == value
                    {
                        stack.push(neighbor);
                        output_stack.insert(neighbor);
                    }
                }
            }

            let output_stack: Vec<Coord> = output_stack.into_iter().collect();

            let mut perimeter = vec![];
            for &current in &output_stack {
                for &direction in QuadDirection::get_all_directions() {
                    let neighbor = current + direction;
                    if !grid.is_coord_in_bounds(neighbor) || grid[neighbor] != value {
                        perimeter.push((current, direction));
                    }
                }
            }

            // deduplicate "faces" into edges

            #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
            struct Edge {
                from: Coord,
                to: Coord,
                facing: QuadDirection,
            }

            impl Edge {
                fn combine_edges(self, other: Self) -> Option<Self> {
                    if self.facing != other.facing {
                        return None;
                    }

                    if self.from.is_adjacent(other.to) || self.to.is_adjacent(other.from) {
                        let (from, to) = [(other.from, self.to), (other.to, self.from)]
                            .iter()
                            .copied()
                            .max_by_key(|&(x, y)| x.manhattan_distance(y))
                            .unwrap();

                        return Some(Edge {
                            from,
                            to,
                            facing: self.facing,
                        });
                    }

                    None
                }
            }

            perimeter.sort();

            let edges: Vec<Edge> = perimeter
                .iter()
                .map(|&(coord, direction)| Edge {
                    from: coord,
                    to: coord,
                    facing: direction,
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

            perimeter_sum += edges.len() * output_stack.len();
        }

        perimeter_sum.to_string()
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
        let mut solution = Day12::new();

        assert_eq!(solution.part1(r#"AA"#), String::from("12"));

        assert_eq!(
            solution.part1(
                r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#
            ),
            String::from("772")
        );

        assert_eq!(
            solution.part1(
                r#"AAAA
BBCD
BBCC
EEEC"#
            ),
            String::from("140")
        );

        assert_eq!(
            solution.part1(
                r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#
            ),
            String::from("1930")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day12::new();

        assert_eq!(solution.part2(r#"AAAA"#), String::from("16"));

        assert_eq!(
            solution.part2(
                r#"AAAA
BBCD
BBCC
EEEC"#
            ),
            String::from("80")
        );

        assert_eq!(
            solution.part2(
                r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#
            ),
            String::from("368")
        );
    }
}
