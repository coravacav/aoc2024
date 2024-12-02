use ahash::AHashMap;

use crate::Solution;

pub struct Day1 {}

impl Solution for Day1 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let (mut left, mut right) = input
            .lines()
            .flat_map(|line| line.split_once("   "))
            .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
            .unzip::<_, _, Vec<_>, Vec<_>>();

        left.sort();
        right.sort();

        left.iter()
            .copied()
            .zip(right.iter().copied())
            .map(|(l, r)| l.abs_diff(r))
            .sum::<u32>()
            .to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        Some(String::from("1830467"))
    }

    fn part2(&mut self, input: &str) -> String {
        let mut left = vec![];
        let mut freq = AHashMap::new();

        for (l, r) in input.lines().flat_map(|line| line.split_once("   ")) {
            left.push(l.parse::<u32>().unwrap());
            *freq.entry(r.parse::<u32>().unwrap()).or_insert(0) += 1;
        }

        left.iter()
            .map(|l| l * freq.get(l).unwrap_or(&0))
            .sum::<u32>()
            .to_string()
    }

    fn known_solution_part2(&self) -> Option<String> {
        Some(String::from("26674158"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day1::new();
        assert_eq!(
            solution.part1(
                r#"3   4
4   3
2   5
1   3
3   9
3   3"#
            ),
            String::from("11")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day1::new();
        assert_eq!(
            solution.part2(
                r#"3   4
4   3
2   5
1   3
3   9
3   3"#
            ),
            String::from("31")
        );
    }
}
