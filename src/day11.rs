use ahash::AHashMap;
use itertools::Itertools;

use crate::Solution;

pub struct Day11 {}

fn split_number(num: u64) -> Option<(u64, u64)> {
    let digits = num.ilog10() + 1;
    if digits % 2 == 1 {
        return None;
    }
    let half = digits / 2;
    let divisor = 10u32.pow(half);

    let first_part = num / divisor as u64;
    let second_part = num % divisor as u64;

    Some((first_part, second_part))
}

pub fn do_stone(stone: u64, depth: u16, saved_stones: &mut AHashMap<(u64, u16), u64>) -> u64 {
    if depth == 0 {
        return 1;
    }

    if let Some(result) = saved_stones.get(&(stone, depth)) {
        return *result;
    }

    let sum = {
        if stone == 0 {
            do_stone(1, depth - 1, saved_stones)
        } else if let Some((left, right)) = split_number(stone) {
            do_stone(left, depth - 1, saved_stones) + do_stone(right, depth - 1, saved_stones)
        } else {
            do_stone(stone * 2024, depth - 1, saved_stones)
        }
    };

    saved_stones.insert((stone, depth), sum);
    sum
}

impl Solution for Day11 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let c = input
            .split(" ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect_vec();

        let mut suma = 0;

        for i in c {
            suma += do_stone(i, 25, &mut AHashMap::new());
        }

        suma.to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        Some(String::from("175006"))
    }

    fn part2(&mut self, input: &str) -> String {
        let c = input
            .split(" ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect_vec();

        let mut suma = 0;

        for i in c {
            suma += do_stone(i, 75, &mut AHashMap::new());
        }

        suma.to_string()
    }

    fn known_solution_part2(&self) -> Option<String> {
        Some(String::from("207961583799296"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_sol() {
        let mut solution = Day11::new();
        assert_eq!(solution.part1(r#"125 17"#), String::from("55312"));
    }
}
