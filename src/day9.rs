use std::num::NonZeroU16;

use itertools::Itertools;

use crate::Solution;

pub struct Day9 {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    File(NonZeroU16),
    Empty,
}

impl Solution for Day9 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let mut file_id = NonZeroU16::new(1).unwrap();

        assert!(size_of::<Space>() == size_of::<u16>());

        let arr = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .enumerate()
            .map(|(i, c)| {
                (
                    if i % 2 == 0 {
                        let new = Space::File(file_id);
                        file_id = file_id.checked_add(1).unwrap();
                        new
                    } else {
                        Space::Empty
                    },
                    c,
                )
            })
            .flat_map(|(file, c)| std::iter::repeat_n(file, c))
            .collect_vec();

        let mut read_backwards_files = arr
            .iter()
            .enumerate()
            .rev()
            .filter(|s| matches!(*s.1, Space::File(_)));

        let mut stop_index = arr.len();

        arr.iter()
            .enumerate()
            .map_while(|(i, s)| match s {
                Space::File(c) if i <= stop_index => Some((i, Space::File(*c))),
                _ => {
                    let (index, next) = read_backwards_files.next().unwrap();
                    stop_index = index - 1;
                    if i >= index || i >= stop_index {
                        None
                    } else {
                        Some((i, *next))
                    }
                }
            })
            .map(|(i, s)| match s {
                Space::File(c) => (c.get() as usize - 1) * i,
                _ => unreachable!(),
            })
            .sum::<usize>()
            .to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        Some(String::from("6448989155953"))
    }

    fn part2(&mut self, input: &str) -> String {
        let mut file_id = NonZeroU16::new(1).unwrap();

        let mut arr = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .enumerate()
            .map(|(i, c)| {
                (
                    if i % 2 == 0 {
                        let new = Space::File(file_id);
                        file_id = file_id.checked_add(1).unwrap();
                        new
                    } else {
                        Space::Empty
                    },
                    c,
                )
            })
            .flat_map(|(file, c)| std::iter::repeat_n(file, c))
            .collect_vec();

        let binding = arr
            .iter()
            .enumerate()
            .rev()
            .filter(|s| matches!(*s.1, Space::File(_)))
            .chunk_by(|s| match *s.1 {
                Space::File(c) => c,
                _ => unreachable!(),
            });

        let chunks = binding
            .into_iter()
            .map(|(file_id, v)| (file_id, v.map(|(idx, space)| (idx, *space)).collect_vec()))
            .collect_vec();

        for (file_id, file_chunk) in chunks {
            // find first empty chunk of size file_chunk.len() - 1

            let chunk_by = arr
                .iter()
                .enumerate()
                .chunk_by(|(_, s)| **s == Space::Empty);

            let indexes_to_swap = chunk_by
                .into_iter()
                .filter(|(is_empty, _)| *is_empty)
                .map(|(_, v)| v.collect_vec())
                .find(|v| v.len() >= file_chunk.len())
                .into_iter()
                .flat_map(|a| a.into_iter().map(|(index, _)| index).take(file_chunk.len()))
                .collect_vec();

            // if the index swap would result in a right shift, stop.
            let Some(first_index_swap_destintaion) = indexes_to_swap.first() else {
                continue;
            };

            if *first_index_swap_destintaion >= file_chunk[0].0 {
                continue;
            }

            for index in indexes_to_swap {
                arr[index] = Space::File(file_id);
            }

            for (index, _) in file_chunk {
                arr[index] = Space::Empty;
            }
        }

        // calculate checksum

        arr.iter()
            .enumerate()
            .filter(|(_, s)| **s != Space::Empty)
            .map(|(i, s)| match s {
                Space::File(c) => (c.get() as usize - 1) * i,
                _ => unreachable!(),
            })
            .sum::<usize>()
            .to_string()
    }

    fn known_solution_part2(&self) -> Option<String> {
        Some(String::from("6476642796832"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day9::new();
        assert_eq!(
            solution.part1(r#"2333133121414131402"#),
            String::from("1928")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day9::new();
        assert_eq!(
            solution.part2(r#"2333133121414131402"#),
            String::from("2858")
        );
    }
}
