pub trait Solution {
    fn new() -> Self
    where
        Self: Sized;
    fn part1(&mut self, input: &str) -> String;
    fn part2(&mut self, input: &str) -> String;

    fn known_solution_part1(&self) -> Option<String> {
        None
    }

    fn known_solution_part2(&self) -> Option<String> {
        None
    }
}

pub mod day1;
pub mod day2;
pub mod day3;
// pub mod day10;
// pub mod day11;
// pub mod day12;
// pub mod day13;
// pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;
// pub mod day4;
// pub mod day5;
// pub mod day6;
// pub mod day7;
// pub mod day8;
// pub mod day9;

pub fn get_solution(day: u8) -> Box<dyn Solution> {
    match day {
        1 => Box::new(day1::Day1::new()),
        2 => Box::new(day2::Day2::new()),
        3 => Box::new(day3::Day3::new()),
        // 4 => day4::Day4::new(),
        // 5 => day5::Day5::new(),
        // 6 => day6::Day6::new(),
        // 7 => day7::Day7::new(),
        // 8 => day8::Day8::new(),
        // 9 => day9::Day9::new(),
        // 10 => day10::Day10::new(),
        // 11 => day11::Day11::new(),
        // 12 => day12::Day12::new(),
        // 13 => day13::Day13::new(),
        // 14 => day14::Day14::new(),
        // 15 => day15::Day15::new(),
        // 16 => day16::Day16::new(),
        // 17 => day17::Day17::new(),
        // 18 => day18::Day18::new(),
        // 19 => day19::Day19::new(),
        // 20 => day20::Day20::new(),
        // 21 => day21::Day21::new(),
        // 22 => day22::Day22::new(),
        // 23 => day23::Day23::new(),
        // 24 => day24::Day24::new(),
        // 25 => day25::Day25::new(),
        _ => panic!("Invalid day"),
    }
}

pub fn get_input(day: u8) -> String {
    // File path is in inputs/{}_input.txt
    std::fs::read_to_string(format!("inputs/{}_input.txt", day)).unwrap()
}
