use crate::Solution;

pub struct Day19 {}

impl Solution for Day19 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let _ = input;
        String::new()
    }

    fn known_solution_part1(&self) -> Option<String> {
        None
    }

    fn part2(&mut self, input: &str) -> String {
        let _ = input;
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
        let mut solution = Day19::new();
        assert_eq!(solution.part1(r#""#), String::from(""));
    }

    #[test]
    fn test_part2() {
        let mut solution = Day19::new();
        assert_eq!(solution.part2(r#""#), String::from(""));
    }
}
