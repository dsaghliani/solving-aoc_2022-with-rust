use itertools::Itertools;

#[must_use]
#[allow(clippy::missing_panics_doc, clippy::expect_used)]
pub fn process_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|sections| {
                    sections
                        .split('-')
                        .map(|section_id| {
                            section_id
                                .parse::<u32>()
                                .expect("Section IDs should be unsigned integers.")
                        })
                        .collect_tuple()
                        .expect("Covered sections should be described with a '-'.")
                })
                .collect_tuple()
                .expect("Pairs of covered sections should be separated by a ','.")
        })
        .filter(|((min1, max1), (min2, max2))| {
            (min1..=max1).contains(&min2) && (min1..=max1).contains(&max2)
                || (min2..=max2).contains(&min1) && (min2..=max2).contains(&max1)
        })
        .count()
}

#[must_use]
#[allow(clippy::missing_panics_doc, clippy::expect_used)]
pub fn process_part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|sections| {
                    sections
                        .split('-')
                        .map(|section_id| {
                            section_id
                                .parse::<u32>()
                                .expect("Section IDs should be unsigned integers.")
                        })
                        .collect_tuple()
                        .expect("Covered sections should be described with a '-'.")
                })
                .collect_tuple()
                .expect("Pairs of covered sections should be separated by a ','.")
        })
        .filter(|((min1, max1), (min2, max2))| {
            (min1..=max1).contains(&min2)
                || (min1..=max1).contains(&max2)
                || (min2..=max2).contains(&min1)
                || (min2..=max2).contains(&max1)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::{process_part_1, process_part_2};
    use std::fs;

    #[test]
    fn part_1_is_2() {
        let input = get_input();
        assert_eq!(process_part_1(&input), 2);
    }

    #[test]
    fn part_2_is_4() {
        let input = get_input();
        assert_eq!(process_part_2(&input), 4);
    }

    #[allow(clippy::unwrap_used)]
    fn get_input() -> String {
        fs::read_to_string("test_input").unwrap()
    }
}
