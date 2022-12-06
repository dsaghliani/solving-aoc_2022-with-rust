#[must_use]
#[allow(clippy::missing_panics_doc, clippy::expect_used)]
pub fn process_part_1(input: &str) -> usize {
    find_how_long_til_n_unique_chars(input, 4).expect("There should always be an answer.")
}

#[must_use]
#[allow(clippy::missing_panics_doc, clippy::expect_used)]
pub fn process_part_2(input: &str) -> usize {
    find_how_long_til_n_unique_chars(input, 14).expect("There should always be an answer.")
}

fn find_how_long_til_n_unique_chars(input: &str, n: usize) -> Option<usize> {
    use std::collections::HashSet;

    input
        .as_bytes()
        .windows(n)
        .position(|chars| {
            let set: HashSet<_> = chars.iter().collect();
            set.len() == n
        })
        .map(|idx| idx + n)
}

#[cfg(test)]
mod tests {
    use super::{process_part_1, process_part_2};

    // PART 1
    #[test]
    fn part_1_test_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(process_part_1(input), 7);
    }

    #[test]
    fn part_1_test_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(process_part_1(input), 5);
    }

    #[test]
    fn part_1_test_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(process_part_1(input), 6);
    }

    #[test]
    fn part_1_test_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(process_part_1(input), 10);
    }

    #[test]
    fn part_1_test_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(process_part_1(input), 11);
    }

    // PART 2
    #[test]
    fn part_2_test_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(process_part_2(input), 19);
    }

    #[test]
    fn part_2_test_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(process_part_2(input), 23);
    }

    #[test]
    fn part_2_test_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(process_part_2(input), 23);
    }

    #[test]
    fn part_2_test_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(process_part_2(input), 29);
    }

    #[test]
    fn part_2_test_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(process_part_2(input), 26);
    }
}
