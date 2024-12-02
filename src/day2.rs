use itertools::Itertools;

use crate::Solution;

pub struct Day2 {}

fn check<const SKIP_ONE: bool>(l: Vec<u16>) -> bool {
    let mut is_asc = false;
    let mut is_desc = false;

    let mut indexes_to_skip: Option<[usize; 3]> = None;

    for (idx, (a, b)) in l.iter().tuple_windows().enumerate() {
        if a < b && !is_desc {
            is_asc = true;
            match b - a {
                1..=3 => {}
                _ => {
                    if SKIP_ONE {
                        indexes_to_skip = Some([idx.saturating_sub(1), idx, idx + 1]);
                        break;
                    }

                    return false;
                }
            }
        } else if a > b && !is_asc {
            is_desc = true;
            match a - b {
                1..=3 => {}
                _ => {
                    if SKIP_ONE {
                        indexes_to_skip = Some([idx.saturating_sub(1), idx, idx + 1]);
                        break;
                    }

                    return false;
                }
            }
        } else {
            if SKIP_ONE {
                indexes_to_skip = Some([idx.saturating_sub(1), idx, idx + 1]);
                break;
            }

            return false;
        }
    }

    if let Some(indexes_to_skip) = indexes_to_skip {
        for idx in indexes_to_skip {
            let mut n = l.clone();
            if n.get(idx).is_none() {
                continue;
            }

            n.remove(idx);
            if check::<false>(n) {
                return true;
            }
        }

        false
    } else {
        true
    }
}

impl Solution for Day2 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        input
            .lines()
            .map(|line| -> u16 {
                check::<false>(
                    line.split(" ")
                        .map(|s| s.parse::<u16>().unwrap())
                        .collect_vec(),
                )
                .into()
            })
            .sum::<u16>()
            .to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        Some("202".to_string())
    }

    fn part2(&mut self, input: &str) -> String {
        input
            .lines()
            .map(|line| -> u16 {
                check::<true>(
                    line.split(" ")
                        .map(|s| s.parse::<u16>().unwrap())
                        .collect_vec(),
                )
                .into()
            })
            .sum::<u16>()
            .to_string()
    }

    fn known_solution_part2(&self) -> Option<String> {
        Some("271".to_string())
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
