use std::{
    collections::{BTreeSet, BinaryHeap, HashMap},
    hint::black_box,
};

use ahash::AHashMap;
use divan::Bencher;
use rand::Rng;

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
    let random_numbers: Vec<u32> = (0..1000000)
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
