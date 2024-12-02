use itertools::Itertools;

use crate::Solution;

pub struct Day2 {}

fn check(line: &[i32]) -> u32 {
    // if all pos
    if line.iter().all(|a| a.is_positive()) {
        return line.iter().all(|&a| a == 1 || a == 2 || a == 3).into();
    }

    if line.iter().all(|a| a.is_negative()) {
        return line.iter().all(|&a| a == -1 || a == -2 || a == -3).into();
    }

    0
}

impl Solution for Day2 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let line = line
                    .split(" ")
                    .map(|s| s.parse::<i32>().unwrap())
                    .tuple_windows()
                    .map(|(a, b)| a - b)
                    .collect_vec();

                check(&line)
            })
            .sum::<u32>()
            .to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        None
    }

    fn part2(&mut self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let line = line
                    .split(" ")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect_vec();

                if check(
                    &line
                        .iter()
                        .tuple_windows()
                        .map(|(a, b)| a - b)
                        .collect_vec(),
                ) == 1
                {
                    return 1;
                }

                // try dropping one of the numbers each
                for i in 0..line.len() {
                    let mut new_line = line.clone();
                    new_line.remove(i);
                    if check(
                        &new_line
                            .iter()
                            .tuple_windows()
                            .map(|(a, b)| a - b)
                            .collect_vec(),
                    ) == 1
                    {
                        return 1;
                    }
                }

                if check(&line) == 1 {
                    return 1;
                }

                0
            })
            .sum::<u32>()
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
        let mut solution = Day2::new();
        assert_eq!(
            solution.part1(
                r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#
            ),
            String::from("2")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day2::new();
        assert_eq!(
            solution.part2(
                r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#
            ),
            String::from("4")
        );
    }
}
