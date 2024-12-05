use std::{
    collections::{BTreeSet, BinaryHeap, HashMap},
    hint::black_box,
    iter::{once, repeat_n},
};

use ahash::AHashMap;
use anyhow::Result;
use aoc2024::{Solution, day3::Day3, day4::Day4};
use divan::Bencher;
use itertools::{Itertools, izip};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::char,
    combinator::{map, opt, rest},
    multi::fold_many1,
    sequence::{preceded, terminated, tuple},
};
use rand::Rng;
use regex::Regex;

fn main() {
    divan::main();
}

#[derive(Debug, Clone, Copy)]
enum SortImplementationText {
    Vec,
    BinaryHeap,
    BTreeSet,
}

#[divan::bench(args = [SortImplementationText::Vec, SortImplementationText::BinaryHeap, SortImplementationText::BTreeSet])]
fn sort_bench(bencher: Bencher, implementation: SortImplementationText) {
    let random_numbers: Vec<u32> = (0..10000)
        .map(|_| rand::thread_rng().gen_range(0..100))
        .collect();

    let random_numbers = black_box(random_numbers);

    let implementation: &mut dyn FnMut() -> String = match implementation {
        SortImplementationText::BinaryHeap => &mut || {
            let collection = black_box(random_numbers.iter())
                .copied()
                .collect::<BinaryHeap<_>>();
            collection
                .into_sorted_vec()
                .iter()
                .copied()
                .sum::<u32>()
                .to_string()
        },
        SortImplementationText::Vec => &mut || {
            let mut collection = black_box(random_numbers.iter())
                .copied()
                .collect::<Vec<_>>();
            collection.sort();
            collection.iter().copied().sum::<u32>().to_string()
        },
        SortImplementationText::BTreeSet => &mut || {
            let collection = black_box(random_numbers.iter())
                .copied()
                .collect::<BTreeSet<_>>();

            collection.into_iter().sum::<u32>().to_string()
        },
    };

    bencher.bench_local(move || {
        black_box(implementation());
    });
}

#[derive(Debug, Clone, Copy)]
enum Day1Part1Implementation {
    Vec,
    BinaryHeap,
}

#[divan::bench(args = [Day1Part1Implementation::Vec, Day1Part1Implementation::BinaryHeap])]
fn day1_part1_bench(bencher: Bencher, implementation: Day1Part1Implementation) {
    let input = black_box(include_str!("../inputs/1_input.txt"));

    let implementation: &mut dyn FnMut() -> String = match implementation {
        Day1Part1Implementation::BinaryHeap => &mut || {
            let (left, right) = input
                .lines()
                .flat_map(|line| line.split_once("   "))
                .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
                .unzip::<_, _, BinaryHeap<_>, BinaryHeap<_>>();

            left.into_sorted_vec()
                .iter()
                .copied()
                .zip(right.into_sorted_vec().iter().copied())
                .map(|(l, r)| l.abs_diff(r))
                .sum::<u32>()
                .to_string()
        },
        Day1Part1Implementation::Vec => &mut || {
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
        },
    };

    bencher.bench_local(move || {
        black_box(implementation());
    });
}

#[derive(Debug, Clone, Copy)]
enum Day1Part2Implementation {
    Vec,
    Hash,
    StdLibHash,
}

#[divan::bench(args = [Day1Part2Implementation::Vec, Day1Part2Implementation::StdLibHash, Day1Part2Implementation::Hash])]
fn day1_part2_bench(bencher: Bencher, implementation: Day1Part2Implementation) {
    let input = black_box(include_str!("../inputs/1_input.txt"));

    let implementation: &mut dyn FnMut() -> String = match implementation {
        Day1Part2Implementation::Hash => &mut || {
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
        },
        Day1Part2Implementation::StdLibHash => &mut || {
            let mut left = vec![];
            let mut freq = HashMap::new();

            for (l, r) in input.lines().flat_map(|line| line.split_once("   ")) {
                left.push(l.parse::<u32>().unwrap());
                *freq.entry(r.parse::<u32>().unwrap()).or_insert(0) += 1;
            }

            left.iter()
                .map(|l| l * freq.get(l).unwrap_or(&0))
                .sum::<u32>()
                .to_string()
        },
        Day1Part2Implementation::Vec => &mut || {
            let (left, right) = input
                .lines()
                .flat_map(|line| line.split_once("   "))
                .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
                .unzip::<_, _, Vec<_>, Vec<_>>();

            left.iter()
                .map(|i| i * right.iter().filter(|x| *x == i).count() as u32)
                .sum::<u32>()
                .to_string()
        },
    };

    bencher.bench_local(move || {
        black_box(implementation());
    });
}

#[divan::bench(args = [Day1Part2Implementation::Vec, Day1Part2Implementation::StdLibHash, Day1Part2Implementation::Hash])]
fn day1_part2_bench_non_build(bencher: Bencher, implementation: Day1Part2Implementation) {
    let input = black_box(include_str!("../inputs/1_input.txt"));

    let mut freq = AHashMap::new();
    let mut std_lib_freq = HashMap::new();

    for (_, r) in input.lines().flat_map(|line| line.split_once("   ")) {
        *freq.entry(r.parse::<u32>().unwrap()).or_insert(0) += 1;
        *std_lib_freq.entry(r.parse::<u32>().unwrap()).or_insert(0) += 1;
    }

    let (left, right) = input
        .lines()
        .flat_map(|line| line.split_once("   "))
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let freq = black_box(freq);
    let left = black_box(left);
    let right = black_box(right);
    let std_lib_freq = black_box(std_lib_freq);

    let implementation: &mut dyn FnMut() -> String = match implementation {
        Day1Part2Implementation::Hash => &mut || {
            left.iter()
                .map(|l| l * freq.get(l).unwrap_or(&0))
                .sum::<u32>()
                .to_string()
        },
        Day1Part2Implementation::StdLibHash => &mut || {
            left.iter()
                .map(|l| l * std_lib_freq.get(l).unwrap_or(&0))
                .sum::<u32>()
                .to_string()
        },
        Day1Part2Implementation::Vec => &mut || {
            left.iter()
                .map(|i| i * right.iter().filter(|x| *x == i).count() as u32)
                .sum::<u32>()
                .to_string()
        },
    };

    bencher.bench_local(move || {
        black_box(implementation());
    });
}

#[derive(Debug, Clone, Copy)]
enum Day2Implementation {
    Brute,
    Smartish,
    SmartishPart1Opt,
    OrigamiDuck,
    OrigamiDuckPart2Again,
    Nock,
}

fn brute_check(line: &[i32]) -> u32 {
    // if all pos
    if line.iter().all(|a| a.is_positive()) {
        return line.iter().all(|&a| a == 1 || a == 2 || a == 3).into();
    }

    if line.iter().all(|a| a.is_negative()) {
        return line.iter().all(|&a| a == -1 || a == -2 || a == -3).into();
    }

    0
}

fn smartish_check_part_1(l: Vec<u8>) -> bool {
    let mut is_asc = false;
    let mut is_desc = false;

    for (a, b) in l.iter().tuple_windows() {
        if a < b && !is_desc {
            is_asc = true;
            match b - a {
                1..=3 => {}
                _ => {
                    return false;
                }
            }
        } else if a > b && !is_asc {
            is_desc = true;
            match a - b {
                1..=3 => {}
                _ => {
                    return false;
                }
            }
        } else {
            return false;
        }
    }

    true
}

fn smartish_check<const SKIP_ONE: bool>(l: Vec<u8>) -> bool {
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
            if smartish_check::<false>(n) {
                return true;
            }
        }

        false
    } else {
        true
    }
}

#[divan::bench(args = [Day2Implementation::Brute, Day2Implementation::Smartish, Day2Implementation::OrigamiDuck, Day2Implementation::SmartishPart1Opt, Day2Implementation::Nock])]
fn day2_part1_bench(bencher: Bencher, implementation: Day2Implementation) {
    let input = black_box(include_str!("../inputs/2_input.txt"));

    let implementation: &mut dyn FnMut() -> String = match implementation {
        Day2Implementation::OrigamiDuckPart2Again => &mut || todo!(),
        Day2Implementation::Nock => &mut || {
            #[derive(Debug, PartialEq)]
            enum Direction {
                Inc,
                Dec,
            }

            fn part_a(input: &str) -> usize {
                parse(input)
                    .into_iter()
                    .filter(|report| is_safe(report))
                    .count()
            }

            fn parse(input: &str) -> Vec<Vec<i32>> {
                input
                    .lines()
                    .map(|line| {
                        line.split(" ")
                            .map(|num| num.parse::<i32>().unwrap())
                            .collect()
                    })
                    .collect::<Vec<_>>()
            }

            fn is_safe(vec: &[i32]) -> bool {
                let dir = if vec[0] < vec[1] {
                    Direction::Inc
                } else {
                    Direction::Dec
                };

                vec.windows(2).all(|w| match dir {
                    Direction::Inc => w[0] < w[1] && w[1] - w[0] <= 3,
                    Direction::Dec => w[0] > w[1] && w[0] - w[1] <= 3,
                })
            }

            part_a(input).to_string()
        },
        Day2Implementation::SmartishPart1Opt => &mut || {
            input
                .lines()
                .map(|line| -> u16 {
                    smartish_check_part_1(
                        line.split(" ")
                            .map(|s| s.parse::<u8>().unwrap())
                            .collect_vec(),
                    )
                    .into()
                })
                .sum::<u16>()
                .to_string()
        },
        Day2Implementation::Brute => &mut || {
            input
                .lines()
                .map(|line| {
                    let line = line
                        .split(" ")
                        .map(|s| s.parse::<i32>().unwrap())
                        .tuple_windows()
                        .map(|(a, b)| a - b)
                        .collect_vec();

                    brute_check(&line)
                })
                .sum::<u32>()
                .to_string()
        },
        Day2Implementation::Smartish => &mut || {
            input
                .lines()
                .map(|line| -> u16 {
                    smartish_check::<false>(
                        line.split(" ")
                            .map(|s| s.parse::<u8>().unwrap())
                            .collect_vec(),
                    )
                    .into()
                })
                .sum::<u16>()
                .to_string()
        },
        Day2Implementation::OrigamiDuck => &mut || {
            input
                .lines()
                .map(|line| -> u16 {
                    line.split(" ")
                        .map(|x| x.parse::<i32>().expect("item is number"))
                        .tuple_windows()
                        .map(|(a, b)| a - b)
                        .tuple_windows()
                        .all(|(a, b)| {
                            (1..=3).contains(&a.abs())
                                && (1..=3).contains(&b.abs())
                                && (a.signum() == b.signum())
                        })
                        .into()
                })
                .sum::<u16>()
                .to_string()
        },
    };

    bencher.bench_local(move || {
        black_box(implementation());
    });
}

#[divan::bench(args = [Day2Implementation::Brute, Day2Implementation::Smartish, Day2Implementation::OrigamiDuck, Day2Implementation::OrigamiDuckPart2Again])]
fn day2_part2_bench(bencher: Bencher, implementation: Day2Implementation) {
    let input = black_box(include_str!("../inputs/2_input.txt"));

    let implementation: &mut dyn FnMut() -> String = match implementation {
        Day2Implementation::SmartishPart1Opt => &mut || todo!(),
        Day2Implementation::Nock => &mut || todo!(),
        Day2Implementation::Brute => &mut || {
            input
                .lines()
                .map(|line| {
                    let line = line
                        .split(" ")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect_vec();

                    if brute_check(
                        &line
                            .iter()
                            .tuple_windows()
                            .map(|(a, b)| a - b)
                            .collect_vec(),
                    ) == 1
                    {
                        return 1;
                    }

                    // try dropping one of the numbers each
                    for i in 0..line.len() {
                        let mut new_line = line.clone();
                        new_line.remove(i);
                        if brute_check(
                            &new_line
                                .iter()
                                .tuple_windows()
                                .map(|(a, b)| a - b)
                                .collect_vec(),
                        ) == 1
                        {
                            return 1;
                        }
                    }

                    if brute_check(&line) == 1 {
                        return 1;
                    }

                    0
                })
                .sum::<u32>()
                .to_string()
        },
        Day2Implementation::Smartish => &mut || {
            input
                .lines()
                .map(|line| -> u16 {
                    smartish_check::<true>(
                        line.split(" ")
                            .map(|s| s.parse::<u8>().unwrap())
                            .collect_vec(),
                    )
                    .into()
                })
                .sum::<u16>()
                .to_string()
        },
        Day2Implementation::OrigamiDuck => &mut || {
            input
                .lines()
                .map(|line| -> u16 {
                    let levels: Vec<_> = line
                        .split(" ")
                        .map(|x| x.parse::<i32>().expect("item is number"))
                        .collect();
                    once(levels.iter().collect())
                        .chain(levels.iter().combinations(levels.len() - 1))
                        .any(|level_subset| {
                            level_subset
                                .into_iter()
                                .tuple_windows()
                                .map(|(a, b)| a - b)
                                .tuple_windows()
                                .all(|(a, b): (i32, i32)| {
                                    (1..=3).contains(&a.abs())
                                        && (1..=3).contains(&b.abs())
                                        && (a.signum() == b.signum())
                                })
                        })
                        .into()
                })
                .sum::<u16>()
                .to_string()
        },
        Day2Implementation::OrigamiDuckPart2Again => &mut || {
            input
                .lines()
                .filter(|line| {
                    let levels: Vec<_> = line
                        .split(" ")
                        .map(|x| x.parse::<i32>().expect("item is number"))
                        .collect();
                    once(levels.iter().collect())
                        .chain(levels.iter().combinations(levels.len() - 1))
                        .any(|level_subset| {
                            level_subset
                                .into_iter()
                                .tuple_windows()
                                .map(|(a, b)| a - b)
                                .tuple_windows()
                                .all(|(a, b): (i32, i32)| {
                                    (1..=3).contains(&a.abs())
                                        && (1..=3).contains(&b.abs())
                                        && (a.signum() == b.signum())
                                })
                        })
                })
                .count()
                .to_string()
        },
    };

    bencher.bench_local(move || {
        black_box(implementation());
    });
}

#[derive(Debug, Clone, Copy)]
enum Day3Implementation {
    First,
    Chase,
    OrigamiDuck,
    Alpha,
}

#[divan::bench(args = [Day3Implementation::First, Day3Implementation::Chase, Day3Implementation::OrigamiDuck, Day3Implementation::Alpha])]
fn day3_part1_bench(bencher: Bencher, implementation: Day3Implementation) {
    let input = black_box(include_str!("../inputs/3_input.txt"));

    let implementation: &mut dyn FnMut() -> String = match implementation {
        Day3Implementation::First => &mut || {
            let sol = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
            sol.captures_iter(input)
                .map(|capture| {
                    let a = capture[1].parse::<u32>().unwrap();
                    let b = capture[2].parse::<u32>().unwrap();
                    a * b
                })
                .sum::<u32>()
                .to_string()
        },
        Day3Implementation::Chase => &mut || {
            let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

            re.captures_iter(input)
                .map(|captures| {
                    let (_, [a, b]) = captures.extract();
                    a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
                })
                .sum::<usize>()
                .to_string()
        },
        Day3Implementation::OrigamiDuck => &mut || {
            fn valid_body(input: &str) -> IResult<&str, (usize, usize)> {
                map(
                    tuple((
                        nom::character::complete::u32,
                        char(','),
                        nom::character::complete::u32,
                        char(')'),
                    )),
                    |(a, _, b, _)| (a as usize, b as usize),
                )(input)
            }

            fn instruction(input: &str) -> IResult<&str, Option<(usize, usize)>> {
                preceded(
                    tuple((
                        take_until("mul("),
                        tag::<&str, &str, nom::error::Error<&str>>("mul("),
                    )),
                    opt(valid_body),
                )(input)
            }

            fn part1(input: &str) -> anyhow::Result<usize> {
                Ok(fold_many1(
                    instruction,
                    || 0,
                    |acc, instr| {
                        if let Some((a, b)) = instr {
                            acc + a * b
                        } else {
                            acc
                        }
                    },
                )(input)
                .expect("at least one instruction")
                .1)
            }

            part1(input).unwrap().to_string()
        },

        Day3Implementation::Alpha => &mut || {
            use anyhow::Result;
            use nom::character::complete::{char, u64 as parse_u64};
            use nom::sequence::tuple;
            use nom::{Err as NomErr, IResult};

            macro_rules! parse_mul {
                ($input:expr, &mut $sum:ident) => {
                    match parse_mul($input) {
                        Err(NomErr::Incomplete(_)) => break,
                        Err(NomErr::Error(error) | NomErr::Failure(error)) => error.input,
                        Ok((rest, (lhs, rhs))) => {
                            $sum += lhs * rhs;
                            rest
                        }
                    }
                };
            }

            pub fn part_one(mut input: &str) -> Result<u64> {
                let mut sum = 0;

                while let Some(rest) = find_and_skip(input, "mul(") {
                    input = parse_mul!(rest, &mut sum);
                }

                Ok(sum)
            }

            fn find_and_skip<'a>(input: &'a str, pat: &str) -> Option<&'a str> {
                input.find(pat).map(|idx| &input[idx + pat.len()..])
            }

            fn parse_mul(input: &str) -> IResult<&str, (u64, u64)> {
                tuple((parse_u64, char(','), parse_u64, char(')')))(input)
                    .map(|(rest, (lhs, _, rhs, _))| (rest, (lhs, rhs)))
            }

            part_one(input).unwrap().to_string()
        },
    };

    bencher.bench_local(move || {
        assert_eq!(
            black_box(implementation()),
            Day3::new().known_solution_part1().unwrap()
        );
    });
}

#[divan::bench(args = [Day3Implementation::First, Day3Implementation::Chase, Day3Implementation::OrigamiDuck, Day3Implementation::Alpha])]
fn day3_part2_bench(bencher: Bencher, implementation: Day3Implementation) {
    let input = black_box(include_str!("../inputs/3_input.txt"));

    let implementation: &mut dyn FnMut() -> String = match implementation {
        Day3Implementation::First => &mut || {
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
        },
        Day3Implementation::Chase => &mut || {
            let re = Regex::new(r"(mul|do|don't)\((?:(\d{1,3}),(\d{1,3}))?\)").unwrap();

            re.captures_iter(input)
                .scan(true, |toggle, captures| {
                    Some(match &captures[1] {
                        "mul" if *toggle => match (captures.get(2), captures.get(3)) {
                            (Some(a), Some(b)) => Some(
                                a.as_str().parse::<u32>().unwrap()
                                    * b.as_str().parse::<u32>().unwrap(),
                            ),
                            _ => None,
                        },
                        "do" => {
                            *toggle = true;
                            None
                        }
                        "don't" => {
                            *toggle = false;
                            None
                        }
                        _ => None,
                    })
                })
                .flatten()
                .sum::<u32>()
                .to_string()
        },
        Day3Implementation::OrigamiDuck => &mut || {
            fn valid_body(input: &str) -> IResult<&str, (usize, usize)> {
                map(
                    tuple((
                        nom::character::complete::u32,
                        char(','),
                        nom::character::complete::u32,
                        char(')'),
                    )),
                    |(a, _, b, _)| (a as usize, b as usize),
                )(input)
            }

            fn instruction(input: &str) -> IResult<&str, Option<(usize, usize)>> {
                preceded(
                    tuple((
                        take_until("mul("),
                        tag::<&str, &str, nom::error::Error<&str>>("mul("),
                    )),
                    opt(valid_body),
                )(input)
            }

            fn part1(input: &str) -> anyhow::Result<usize> {
                Ok(fold_many1(
                    instruction,
                    || 0,
                    |acc, instr| {
                        if let Some((a, b)) = instr {
                            acc + a * b
                        } else {
                            acc
                        }
                    },
                )(input)
                .expect("at least one instruction")
                .1)
            }

            fn enabled_segment(input: &str) -> IResult<&str, &str> {
                alt((
                    terminated(
                        take_until("don't()"),
                        opt(tuple((
                            tag("don't()"),
                            alt((terminated(take_until("do()"), tag("do()")), rest)),
                        ))),
                    ),
                    // Require at least one character so we don't match empty string
                    take_while1(|_| true),
                ))(input)
            }

            fn part2(input: &str) -> anyhow::Result<usize> {
                Ok(fold_many1(
                    map(enabled_segment, part1),
                    || 0,
                    |acc, segment_sum| acc + segment_sum.expect("can parse segment"),
                )(input)
                .expect("at least one segment")
                .1)
            }

            part2(input).unwrap().to_string()
        },
        Day3Implementation::Alpha => &mut || {
            use anyhow::Result;
            use nom::character::complete::{char, u64 as parse_u64};
            use nom::sequence::tuple;
            use nom::{Err as NomErr, IResult};

            macro_rules! parse_mul {
                ($input:expr, &mut $sum:ident) => {
                    match parse_mul($input) {
                        Err(NomErr::Incomplete(_)) => break,
                        Err(NomErr::Error(error) | NomErr::Failure(error)) => error.input,
                        Ok((rest, (lhs, rhs))) => {
                            $sum += lhs * rhs;
                            rest
                        }
                    }
                };
            }

            pub fn part_two(mut input: &str) -> Result<u64> {
                let mut sum = 0;

                while let Some((rest, mul_or_dont)) = find_mul_or_dont(input) {
                    input = match mul_or_dont {
                        MulOrDont::Mul => parse_mul!(rest, &mut sum),
                        MulOrDont::Dont => match find_and_skip(rest, "do()") {
                            None => break,
                            Some(rest) => rest,
                        },
                    };
                }

                Ok(sum)
            }

            #[derive(Debug)]
            enum MulOrDont {
                Mul,
                Dont,
            }

            fn find_and_skip<'a>(input: &'a str, pat: &str) -> Option<&'a str> {
                input.find(pat).map(|idx| &input[idx + pat.len()..])
            }

            fn find_mul_or_dont(input: &str) -> Option<(&str, MulOrDont)> {
                input
                    .bytes()
                    .enumerate()
                    .find_map(|(idx, byte)| match byte {
                        b'm' => input[idx + 1..]
                            .strip_prefix("ul(")
                            .map(|rest| (rest, MulOrDont::Mul)),
                        b'd' => input[idx + 1..]
                            .strip_prefix("on't()")
                            .map(|rest| (rest, MulOrDont::Dont)),
                        _ => None,
                    })
            }

            fn parse_mul(input: &str) -> IResult<&str, (u64, u64)> {
                tuple((parse_u64, char(','), parse_u64, char(')')))(input)
                    .map(|(rest, (lhs, _, rhs, _))| (rest, (lhs, rhs)))
            }

            part_two(input).unwrap().to_string()
        },
    };

    bencher.bench_local(move || {
        assert_eq!(
            black_box(implementation()),
            Day3::new().known_solution_part2().unwrap()
        );
    });
}

#[derive(Debug, Clone, Copy)]
enum Day4Implementation {
    Mine,
    Alpha,
    OrigamiDuck,
}

#[divan::bench(args = [Day4Implementation::Mine, Day4Implementation::Alpha, Day4Implementation::OrigamiDuck])]
fn day4_part1_bench(bencher: Bencher, implementation: Day4Implementation) {
    let input = black_box(include_str!("../inputs/4_input.txt"));

    let implementation: &mut dyn FnMut() -> String = match implementation {
        Day4Implementation::Mine => &mut || Day4::new().part1(input),
        Day4Implementation::Alpha => &mut || {
            pub fn part_one(input: &str) -> Result<usize> {
                const OFFSETS: &[(isize, isize)] = &[
                    (-1, 0),
                    (0, 1),
                    (1, 0),
                    (0, -1),
                    (-1, -1),
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                ];

                let grid = Grid::new(input.as_ref());

                Ok(grid
                    .coordinates()
                    .filter(|&(row, col)| grid.get(row, col) == Some(b'X'))
                    .map(|(row, col)| {
                        OFFSETS
                            .iter()
                            .filter(|&&(row_offset, col_offset)| {
                                grid.probe_xmas(row, row_offset, col, col_offset)
                            })
                            .count()
                    })
                    .sum())
            }

            #[derive(Debug)]
            struct Grid<'a> {
                bytes: &'a [u8],
                columns: usize,
            }

            impl<'a> Grid<'a> {
                fn new(bytes: &'a [u8]) -> Self {
                    Self {
                        bytes,
                        columns: bytes
                            .iter()
                            .position(|&byte| byte == b'\n')
                            .unwrap_or(bytes.len()),
                    }
                }

                fn coordinates(&self) -> impl Iterator<Item = (usize, usize)> {
                    itertools::iproduct!(0..self.rows(), 0..self.columns)
                }

                fn rows(&self) -> usize {
                    self.bytes.len() / self.columns
                }

                fn get(&self, row: usize, col: usize) -> Option<u8> {
                    self.bytes.get((row * (self.columns + 1)) + col).copied()
                }

                fn probe_xmas(
                    &self,
                    row: usize,
                    row_offset: isize,
                    col: usize,
                    col_offset: isize,
                ) -> bool {
                    fn probe_char(
                        grid: &Grid<'_>,
                        row: usize,
                        row_offset: isize,
                        col: usize,
                        col_offset: isize,
                        ch: u8,
                    ) -> Option<(usize, usize)> {
                        Option::zip(
                            grid.checked_row(row, row_offset),
                            grid.checked_col(col, col_offset),
                        )
                        .filter(|&(row, col)| grid.get(row, col) == Some(ch))
                    }

                    probe_char(self, row, row_offset, col, col_offset, b'M')
                        .and_then(|(row, col)| {
                            probe_char(self, row, row_offset, col, col_offset, b'A')
                        })
                        .and_then(|(row, col)| {
                            probe_char(self, row, row_offset, col, col_offset, b'S')
                        })
                        .is_some()
                }

                fn checked_row(&self, row: usize, offset: isize) -> Option<usize> {
                    row.checked_add_signed(offset)
                        .filter(|&row| row < self.rows())
                }

                fn checked_col(&self, col: usize, offset: isize) -> Option<usize> {
                    col.checked_add_signed(offset)
                        .filter(|&col| col < self.columns)
                }
            }

            part_one(input).unwrap().to_string()
        },
        Day4Implementation::OrigamiDuck => &mut || {
            macro_rules! matches_xmas {
                ($input: expr) => {
                    matches!($input, ('X', 'M', 'A', 'S') | ('S', 'A', 'M', 'X'))
                };
            }
            let line_length = input.find("\n").expect("at least one line");
            type Column = (char, char, char, char);
            input
                .lines()
                .chain(repeat_n(".".repeat(line_length).as_str(), 3))
                .tuple_windows()
                .fold(0, |acc: usize, lines: (&str, &str, &str, &str)| {
                    acc + izip!(
                        lines.0.chars().chain(repeat_n('.', 3)),
                        lines.1.chars().chain(repeat_n('.', 3)),
                        lines.2.chars().chain(repeat_n('.', 3)),
                        lines.3.chars().chain(repeat_n('.', 3)),
                    )
                    .tuple_windows::<(Column, Column, Column, Column)>()
                    .map(|columns| {
                        let horizontal =
                            matches_xmas!((columns.0.0, columns.1.0, columns.2.0, columns.3.0));
                        let vertical = matches_xmas!(columns.0);
                        let right_diagonal =
                            matches_xmas!((columns.0.0, columns.1.1, columns.2.2, columns.3.3));
                        let left_diagonal =
                            matches_xmas!((columns.3.0, columns.2.1, columns.1.2, columns.0.3));
                        horizontal as usize
                            + vertical as usize
                            + right_diagonal as usize
                            + left_diagonal as usize
                    })
                    .sum::<usize>()
                })
                .to_string()
        },
    };

    bencher.bench_local(move || {
        assert_eq!(
            black_box(implementation()),
            Day4::new().known_solution_part1().unwrap()
        );
    });
}

#[divan::bench(args = [Day4Implementation::Mine, Day4Implementation::Alpha, Day4Implementation::OrigamiDuck])]
fn day4_part2_bench(bencher: Bencher, implementation: Day4Implementation) {
    let input = black_box(include_str!("../inputs/4_input.txt"));

    let implementation: &mut dyn FnMut() -> String = match implementation {
        Day4Implementation::Mine => &mut || Day4::new().part2(input),
        Day4Implementation::Alpha => &mut || {
            pub fn part_two(input: &str) -> Result<usize> {
                let grid = Grid::new(input.as_ref());

                Ok(grid
                    .coordinates()
                    .filter(|&(row, col)| grid.get(row, col) == Some(b'A'))
                    .filter(|&(row, col)| {
                        grid.probe_m_s(row, -1, col, 1) && grid.probe_m_s(row, 1, col, 1)
                    })
                    .count())
            }

            #[derive(Debug)]
            struct Grid<'a> {
                bytes: &'a [u8],
                columns: usize,
            }

            impl<'a> Grid<'a> {
                fn new(bytes: &'a [u8]) -> Self {
                    Self {
                        bytes,
                        columns: bytes
                            .iter()
                            .position(|&byte| byte == b'\n')
                            .unwrap_or(bytes.len()),
                    }
                }

                fn coordinates(&self) -> impl Iterator<Item = (usize, usize)> {
                    itertools::iproduct!(0..self.rows(), 0..self.columns)
                }

                fn rows(&self) -> usize {
                    self.bytes.len() / self.columns
                }

                fn get(&self, row: usize, col: usize) -> Option<u8> {
                    self.bytes.get((row * (self.columns + 1)) + col).copied()
                }

                fn probe_m_s(
                    &self,
                    row: usize,
                    row_offset: isize,
                    col: usize,
                    col_offset: isize,
                ) -> bool {
                    Option::zip(
                        Option::zip(
                            self.checked_row(row, row_offset),
                            self.checked_col(col, col_offset),
                        )
                        .and_then(|(row, col)| self.get(row, col)),
                        Option::zip(
                            self.checked_row(row, -row_offset),
                            self.checked_col(col, -col_offset),
                        )
                        .and_then(|(row, col)| self.get(row, col)),
                    )
                    .is_some_and(|chars| matches!(chars, (b'M', b'S') | (b'S', b'M')))
                }

                fn checked_row(&self, row: usize, offset: isize) -> Option<usize> {
                    row.checked_add_signed(offset)
                        .filter(|&row| row < self.rows())
                }

                fn checked_col(&self, col: usize, offset: isize) -> Option<usize> {
                    col.checked_add_signed(offset)
                        .filter(|&col| col < self.columns)
                }
            }

            part_two(input).unwrap().to_string()
        },
        Day4Implementation::OrigamiDuck => &mut || {
            type Column = (char, char, char);
            input
                .lines()
                .tuple_windows()
                .fold(0, |acc: usize, lines: (&str, &str, &str)| {
                    acc + izip!(lines.0.chars(), lines.1.chars(), lines.2.chars())
                        .tuple_windows::<(Column, Column, Column)>()
                        .filter(|columns| {
                            columns.1.1 == 'A'
                                && matches!(
                                    (columns.0, columns.2),
                                    (('M', _, 'M'), ('S', _, 'S'))
                                        | (('S', _, 'S'), ('M', _, 'M'))
                                        | (('M', _, 'S'), ('M', _, 'S'))
                                        | (('S', _, 'M'), ('S', _, 'M'))
                                )
                        })
                        .count()
                })
                .to_string()
        },
    };

    bencher.bench_local(move || {
        assert_eq!(
            black_box(implementation()),
            Day4::new().known_solution_part2().unwrap()
        );
    });
}
