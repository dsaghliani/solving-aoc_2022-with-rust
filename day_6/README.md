# Advent of Code 2022 – Day 06

[See the entire, unedited problem on the official website.](https://adventofcode.com/2022/day/6)

Today's an easy one.

## Part 1

**Problem:** We're given a stream of characters (and I say "stream", but the input is a full, single-line string), and we're to find the (1-based, not 0-based) index of the first character that completes a so-called start-of-packet marker—a 4-character string where each character is unique.

- `mjqjpqmgbljsphdztnvjfqwrcgsmlb`: first marker after character 7—`jpqm` being the marker and `m` the 7th character
- `bvwbjplbgvbhsrlpgdmjqwftvncz`: first marker after character 5
- `nppdvjthqldpwncqszvftbrmjlhg`: first marker after character 6
- `nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg`: first marker after character 10
- `zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw`: first marker after character 11

This is known as a "sliding window" problem. We must first compare elements 1 through 4, then 2 through 5, 3 through 6, and so on until the end.

As it so happens, Rust's standard library comes with a function that does exactly that: [`std::slice::windows()`](https://doc.rust-lang.org/std/primitive.slice.html#method.windows).

```rs
use std::collections::HashSet;

pub fn process_part_1(input: &str) -> usize {
    let window_size = 4;

    input
        .as_bytes()
        .windows(window_size)
        .position(|chars| {
            let set: HashSet<_> = chars.iter().collect();
            set.len() == window_size
        })
        .map(|idx| idx + window_size)
        .expect("There should always be an answer.")
}
```

1. Because `windows()` is a function on slices and not string slices (i.e. not `&str`), we use `as_bytes()` to get a slice. This works only because we know the input is an ASCII string and each character is represented by one byte. That returns an iterator over `u8`'s and not `char`'s, but it makes no difference. (If we were working with more than ASCII, we'd have to call `input.chars().collect::<Vec<_>>()` and proceed with that.)
2. As the name suggests, `Iterator::position()` finds the position—index—of the first occurrence that satisfies our condition.
3. An easy way to check that the characters are unique is to add them to a `HashSet` and confirm its length is the same as the number of characters added.
4. Because we want the end index of the window and not the start (plus 1, as we're counting from 1 and not 0), we add `size` to the result. `Option` implements `map()`, making that trivial.

## Part 2

**Problem:** Exactly the same as above, but `window_size` is 14 instead.

Let's extract the code into a separate function and reuse that.

```rs
pub fn process_part_1(input: &str) -> usize {
    find_how_long_til_n_unique_chars(input, 4).expect("There should always be an answer.")
}

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
```

We're done!
