use itertools::Itertools;

use crate::Solution;

pub struct Day9 {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    File(usize),
    Empty,
}

impl Solution for Day9 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let mut file_id = 0;

        let mut arr = input
            .chars()
            .enumerate()
            .flat_map(|(i, c)| match (i % 2, c) {
                (0, c) => {
                    let new =
                        std::iter::repeat_n(Space::File(file_id), c.to_digit(10).unwrap() as usize);

                    file_id += 1;

                    new
                }
                (1, c) => std::iter::repeat_n(Space::Empty, c.to_digit(10).unwrap() as usize),
                _ => unreachable!(),
            })
            .collect_vec();

        fn is_solved(arr: &[Space]) -> bool {
            let first_space = arr.iter().position(|s| *s == Space::Empty);
            if let Some(first_space) = first_space {
                arr[first_space..].iter().all(|s| *s == Space::Empty)
            } else {
                false
            }
        }

        while !is_solved(&arr) {
            // replace first empty with last file
            let first_empty = arr.iter_mut().position(|s| *s == Space::Empty).unwrap();
            let last_file = arr
                .iter()
                .rposition(|s| matches!(*s, Space::File(_)))
                .unwrap();

            arr.swap(first_empty, last_file);
        }

        // calculate checksum

        arr.iter()
            .take_while(|s| matches!(*s, Space::File(_)))
            .enumerate()
            .map(|(i, s)| match s {
                Space::File(c) => c * i,
                _ => unreachable!(),
            })
            .sum::<usize>()
            .to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        None
    }

    fn part2(&mut self, input: &str) -> String {
        let mut file_id = 0;

        let mut arr = input
            .chars()
            .enumerate()
            .flat_map(|(i, c)| match (i % 2, c) {
                (0, c) => {
                    let new =
                        std::iter::repeat_n(Space::File(file_id), c.to_digit(10).unwrap() as usize);

                    file_id += 1;

                    new
                }
                (1, c) => std::iter::repeat_n(Space::Empty, c.to_digit(10).unwrap() as usize),
                _ => unreachable!(),
            })
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
                Space::File(c) => c * i,
                _ => unreachable!(),
            })
            .sum::<usize>()
            .to_string()
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
