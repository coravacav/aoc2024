use std::{collections::HashMap, hint::black_box};

use ahash::AHashMap;
use divan::Bencher;

fn main() {
    divan::main();
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
