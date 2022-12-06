use day_6::{process_part_1, process_part_2};
use std::fs;

#[allow(clippy::unwrap_used)]
fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!("Part 1: {}", process_part_1(&input));
    println!("Part 2: {}", process_part_2(&input));
}
