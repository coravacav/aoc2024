use regex::Regex;

use crate::Solution;

pub struct Day3 {}

impl Solution for Day3 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let sol = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        sol.captures_iter(input)
            .map(|capture| {
                let a = capture[1].parse::<u32>().unwrap();
                let b = capture[2].parse::<u32>().unwrap();
                a * b
            })
            .sum::<u32>()
            .to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        Some(188116424.to_string())
    }

    fn part2(&mut self, input: &str) -> String {
        let sol = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        let mut new_input = String::new();

        if let Some((a, _)) = input.split_once("don't()") {
            new_input.push_str(a);
        }

        for dontsection in input.split("don't()").skip(1) {
            for dosection in dontsection.split("do()").skip(1) {
                new_input.push_str(dosection);
            }
        }

        sol.captures_iter(&new_input)
            .map(|capture| {
                let a = capture[1].parse::<u32>().unwrap();
                let b = capture[2].parse::<u32>().unwrap();
                a * b
            })
            .sum::<u32>()
            .to_string()
    }

    fn known_solution_part2(&self) -> Option<String> {
        Some(104245808.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day3::new();
        assert_eq!(
            solution.part1(
                r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#
            ),
            String::from("161")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day3::new();
        assert_eq!(
            solution.part2(
                r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#
            ),
            String::from("48")
        );
    }
}
