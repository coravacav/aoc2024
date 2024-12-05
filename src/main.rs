use aoc2024::{get_input, get_solution};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// What day of the AoC is this?
    num: u8,
}

fn main() {
    let cli = Cli::parse();

    let input = get_input(cli.num);
    let input = input.trim();
    let mut solution = get_solution(cli.num);

    let part1 = solution.part1(input);
    if let Some(known_part1) = solution.known_solution_part1() {
        assert_eq!(part1, known_part1);
    }

    println!("Part 1: {}", part1);

    let part2 = solution.part2(input);
    if let Some(known_part2) = solution.known_solution_part2() {
        assert_eq!(part2, known_part2);
    }

    println!("Part 2: {}", part2);
}
