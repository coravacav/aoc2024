use crate::Solution;

use nalgebra::{Matrix2, Vector2};
use regex::Regex;

pub struct Day13 {}

fn round_if_close(x: &f64) -> Option<f64> {
    let rounded = x.round();
    let distance = (x - rounded).abs();

    if distance < 0.001 {
        Some(rounded)
    } else {
        None
    }
}

impl Solution for Day13 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let mut sum = 0;
        for section in input.split("\n\n") {
            let mut lines = section.lines();
            let button_a = lines.next().unwrap().split_once(": ").unwrap().1;
            let button_b = lines.next().unwrap().split_once(": ").unwrap().1;
            let prize = lines.next().unwrap().split_once(": ").unwrap().1;

            let (a_x, a_y) = button_a.split_once(", ").unwrap();
            let a_x = a_x.split_once("+").unwrap().1.parse::<f64>().unwrap();
            let a_y = a_y.split_once("+").unwrap().1.parse::<f64>().unwrap();

            let (b_x, b_y) = button_b.split_once(", ").unwrap();
            let b_x = b_x.split_once("+").unwrap().1.parse::<f64>().unwrap();
            let b_y = b_y.split_once("+").unwrap().1.parse::<f64>().unwrap();

            let (prize_x, prize_y) = prize.split_once(", ").unwrap();
            let prize_x = prize_x.split_once("=").unwrap().1.parse::<f64>().unwrap();
            let prize_y = prize_y.split_once("=").unwrap().1.parse::<f64>().unwrap();

            // Define a 2x2 matrix A
            let a = Matrix2::new(a_x, b_x, a_y, b_y);

            // Define a 2x1 vector b
            let prize = Vector2::new(prize_x, prize_y);

            // Solve for x in Ax = b
            let x = a.try_inverse().map(|inv_a| inv_a * prize).unwrap();

            if x.iter()
                .any(|x| !(0.0..=100.0).contains(x) || (0.001..0.999).contains(&x.fract()))
            {
                continue;
            }

            let a = x[0].round() as usize;
            let b = x[1].round() as usize;

            sum += a * 3 + b;
        }

        sum.to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        Some(String::from("29388"))
    }

    fn part2(&mut self, input: &str) -> String {
        let mut sum = 0;
        for section in input.split("\n\n") {
            let mut lines = section.lines();
            let button_a = lines.next().unwrap().split_once(": ").unwrap().1;
            let button_b = lines.next().unwrap().split_once(": ").unwrap().1;
            let prize = lines.next().unwrap().split_once(": ").unwrap().1;

            let (a_x, a_y) = button_a.split_once(", ").unwrap();
            let a_x = a_x.split_once("+").unwrap().1.parse::<f64>().unwrap();
            let a_y = a_y.split_once("+").unwrap().1.parse::<f64>().unwrap();

            let (b_x, b_y) = button_b.split_once(", ").unwrap();
            let b_x = b_x.split_once("+").unwrap().1.parse::<f64>().unwrap();
            let b_y = b_y.split_once("+").unwrap().1.parse::<f64>().unwrap();

            let (prize_x, prize_y) = prize.split_once(", ").unwrap();
            let prize_x =
                prize_x.split_once("=").unwrap().1.parse::<f64>().unwrap() + 10000000000000.0;
            let prize_y =
                prize_y.split_once("=").unwrap().1.parse::<f64>().unwrap() + 10000000000000.0;

            // Define a 2x2 matrix A
            let a = Matrix2::new(a_x, b_x, a_y, b_y);

            // Define a 2x1 vector b
            let prize = Vector2::new(prize_x, prize_y);

            // Solve for x in Ax = b
            let x = a.try_inverse().map(|inv_a| inv_a * prize).unwrap();

            if x.iter()
                .map(round_if_close)
                .any(|x| !matches!(x, Some(x) if x >= 0.0))
            {
                continue;
            }

            let a = x[0].round() as usize;
            let b = x[1].round() as usize;

            sum += a * 3 + b;
        }

        sum.to_string()
    }

    fn known_solution_part2(&self) -> Option<String> {
        Some(String::from("99548032866004"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day13::new();
        assert_eq!(
            solution.part1(
                r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#
            ),
            String::from("480")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day13::new();
        assert_eq!(
            solution.part2(
                r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#
            ),
            String::from("")
        );
    }
}
