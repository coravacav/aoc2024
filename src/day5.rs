use ahash::AHashMap;
use itertools::Itertools;
use rand::seq::SliceRandom;

use crate::Solution;

pub struct Day5 {}

impl Solution for Day5 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let mut map: AHashMap<u32, Vec<u32>> = AHashMap::new();

        let (rules, checks) = input.split_once("\n\n").unwrap();

        for rule in rules.lines() {
            let (a, b) = rule.split_once("|").unwrap();
            map.entry(a.parse::<u32>().unwrap())
                .or_default()
                .push(b.parse::<u32>().unwrap());
        }

        checks
            .lines()
            .map(|check| {
                check
                    .split(",")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect_vec()
            })
            .filter(|all| {
                for a in 0..all.len() {
                    for b in a..all.len() {
                        if a == b {
                            continue;
                        }

                        let a = all[a];
                        let b = all[b];

                        if map.get(&a).map(|x| x.contains(&b)).unwrap_or(false) {
                            continue;
                        } else {
                            return false;
                        }
                    }
                }

                true
            })
            .map(|all| all[all.len() / 2])
            .sum::<u32>()
            .to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        Some(String::from("4609"))
    }

    fn part2(&mut self, input: &str) -> String {
        let mut map: AHashMap<u32, Vec<u32>> = AHashMap::new();

        let (rules, checks) = input.split_once("\n\n").unwrap();

        for rule in rules.lines() {
            let (a, b) = rule.split_once("|").unwrap();
            map.entry(a.parse::<u32>().unwrap())
                .or_default()
                .push(b.parse::<u32>().unwrap());
        }

        fn check_one(all: &[u32], map: &AHashMap<u32, Vec<u32>>) -> bool {
            if all.is_empty() {
                return true;
            }

            for a in 0..all.len() {
                for b in a..all.len() {
                    if a == b {
                        continue;
                    }

                    let a = all[a];
                    let b = all[b];

                    if map.get(&a).map(|x| x.contains(&b)).unwrap_or(false) {
                        continue;
                    } else {
                        return false;
                    }
                }
            }

            true
        }

        checks
            .lines()
            .map(|check| {
                check
                    .split(",")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect_vec()
            })
            .filter(|all| !check_one(all, &map))
            .map(|mut all| {
                let mut result: Vec<u32> = vec![];

                while !check_one(&all, &map) {
                    // check first for incoming edges
                    let first = all[0];

                    let incoming_count = all
                        .iter()
                        .flat_map(|t| map.get(t).map(|v| (t, v)))
                        .filter(|to| all.contains(to.0))
                        .filter(|to| to.1.contains(&first))
                        .count();

                    if incoming_count == 0 {
                        // pop the first element
                        result.push(all.remove(0));
                    }

                    all.shuffle(&mut rand::thread_rng());
                }

                for x in all {
                    result.push(x);
                }

                result
            })
            .map(|all| all[all.len() / 2])
            .sum::<u32>()
            .to_string()
    }

    fn known_solution_part2(&self) -> Option<String> {
        Some(String::from("5723"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day5::new();
        assert_eq!(
            solution.part1(
                r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#
            ),
            String::from("143")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day5::new();
        assert_eq!(
            solution.part2(
                r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#
            ),
            String::from("123")
        );
    }
}
