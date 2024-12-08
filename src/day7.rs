use crate::Solution;

pub struct Day7 {}

fn recur_equation_solvable(output: usize, inputs: &[usize], carry: usize) -> bool {
    if inputs.is_empty() {
        return output == carry;
    }

    if carry > output {
        return false;
    }

    // try addition or multiplication of the next number
    recur_equation_solvable(output, &inputs[1..], carry + inputs[0])
        || recur_equation_solvable(output, &inputs[1..], carry * inputs[0])
}

fn concat(left: usize, right: usize) -> usize {
    left * 10usize.pow((right as f64).log(10.0).floor() as u32 + 1) + right
}

fn recur_equation_solvable_concat(output: usize, inputs: &[usize], carry: usize) -> bool {
    if inputs.is_empty() {
        return output == carry;
    }

    if carry > output {
        return false;
    }

    // try addition or multiplication of the next number
    recur_equation_solvable_concat(output, &inputs[1..], carry + inputs[0])
        || recur_equation_solvable_concat(output, &inputs[1..], carry * inputs[0])
        || recur_equation_solvable_concat(output, &inputs[1..], concat(carry, inputs[0]))
}

impl Solution for Day7 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        input
            .lines()
            .filter_map(|line| {
                let (output, inputs) = line.split_once(": ").unwrap();
                let inputs: Vec<_> = inputs
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();

                let output = output.parse::<usize>().unwrap();

                recur_equation_solvable(output, &inputs[1..], inputs[0]).then_some(output)
            })
            .sum::<usize>()
            .to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        Some("3312271365652".to_string())
    }

    fn part2(&mut self, input: &str) -> String {
        input
            .lines()
            .filter_map(|line| {
                let (output, inputs) = line.split_once(": ").unwrap();
                let inputs: Vec<_> = inputs
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();

                let output = output.parse::<usize>().unwrap();

                recur_equation_solvable_concat(output, &inputs[1..], inputs[0]).then_some(output)
            })
            .sum::<usize>()
            .to_string()
    }

    fn known_solution_part2(&self) -> Option<String> {
        Some("509463489296712".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day7::new();
        assert_eq!(
            solution.part1(
                r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#
            ),
            String::from("3749")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day7::new();
        assert_eq!(
            solution.part2(
                r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#
            ),
            String::from("11387")
        );
    }
}
