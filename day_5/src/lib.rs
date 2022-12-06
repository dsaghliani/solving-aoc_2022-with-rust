#![feature(iter_array_chunks)]

use itertools::Itertools;
use std::{collections::HashMap, iter};

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn process_part_1(input: &str) -> String {
    let mut crates = parse_crates(input);
    let instructions = parse_instructions(input);

    for (amount, from, to) in instructions {
        let popped_crates = crates.get_mut(&from).map_or_else(
            || panic!("Instruction tried to take from a stack that doesn't exist."),
            |stack| stack.split_off(stack.len() - amount),
        );

        crates.get_mut(&to).map_or_else(
            || panic!("Instruction tried to add to a stack that doesn't exist."),
            |stack| stack.extend(popped_crates.into_iter().rev()),
        );
    }

    get_topmost_crates(crates)
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn process_part_2(input: &str) -> String {
    let mut crates = parse_crates(input);
    let instructions = parse_instructions(input);

    for (amount, from, to) in instructions {
        let mut popped_crates = crates.get_mut(&from).map_or_else(
            || panic!("Instruction tried to take from a stack that doesn't exist."),
            |stack| stack.split_off(stack.len() - amount),
        );

        crates.get_mut(&to).map_or_else(
            || panic!("Instruction tried to add to a stack that doesn't exist."),
            |stack| stack.append(&mut popped_crates),
        );
    }

    get_topmost_crates(crates)
}

fn parse_crates(input: &str) -> HashMap<usize, Vec<char>> {
    input
        .lines()
        .take_while(|line| line.trim_start().starts_with('['))
        .flat_map(|line| {
            line.chars()
                .chain(iter::once(' '))
                .array_chunks::<4>()
                .enumerate()
                .filter_map(|(col, [_, maybe_crate, _, _])| {
                    if maybe_crate == ' ' {
                        None
                    } else {
                        Some((col, maybe_crate))
                    }
                })
        })
        .fold(HashMap::new(), |mut crates, (col, char)| {
            // The instructions use one-based indexing, so adjust `col` accordingly.
            let col = col + 1;

            #[allow(clippy::option_if_let_else)]
            if let Some(chars) = crates.get_mut(&col) {
                chars.insert(0, char);
            } else {
                crates.insert(col, vec![char]);
            }

            crates
        })
}

#[allow(clippy::expect_used)]
fn parse_instructions(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .filter(|line| line.starts_with("move"))
        .map(|line| {
            line.split(' ')
                .filter_map(|str| str.parse::<usize>().ok())
                .collect_tuple()
                .expect("Every instruction should contain 3 numbers.")
        })
        .collect()
}

fn get_topmost_crates(crates: HashMap<usize, Vec<char>>) -> String {
    crates
        .into_iter()
        .sorted()
        .filter_map(|(_col, mut stack)| stack.pop())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{process_part_1, process_part_2};
    use std::fs;

    #[test]
    fn part_1_is_2() {
        let input = get_input();
        assert_eq!(process_part_1(&input), "CMZ".to_owned());
    }

    #[test]
    fn part_2_is_4() {
        let input = get_input();
        assert_eq!(process_part_2(&input), "MCD".to_owned());
    }

    #[allow(clippy::unwrap_used)]
    fn get_input() -> String {
        fs::read_to_string("test_input").unwrap()
    }
}
