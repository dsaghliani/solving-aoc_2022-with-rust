# Advent of Code 2022: Day 04

[See the problem on the Advent of Code website.](https://adventofcode.com/2022/day/4)

## Part 1

**Problem:** Every line contains the sections—represented as a range of unsigned integers—covered by a pair of elves. Calculate the number of instances where the range of one of the elves wholly covers the other's.

Let's break this down into two parts. First, we must extract the textual input into usable form. Checking for overlaps can come afterward.

```rs
use itertools::Itertools;

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
    // ...
}
```
Tedious but straightforward. Notice the use of `collect_tuple()`. That's a convenience function from the trait `itertools::Itertools` that lets us avoid calling `.next().unwrap()` on the iterator twice. Since Rust doesn't allow collecting into arrays, this is the best we can do (that I know of).

Now, it's time to move on to the actual problem. There are several ways to solve it, but let's make use of Rust's built-in ranges.

All we need to find out is how many times overlaps happen. That's a hint, and it tells us, "Use `filter()` and `count()`."

```rs
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
```

The use of `collect_tuple()` lets us pattern-match on the section IDs in the closure signature. All that's left is to check whether each section range contains the start and end of the other.o

- Make sure that the ranges are inclusive—in other words, don't forget the `=` after the `..`.

## Part 2

**Problem**: The same as before, except that partial overlaps should be included, too.

The only change we need to make is replace the and operators (`&&`) with the or ones (`||`). Everything else stays identical.

```rs
.filter(|((min1, max1), (min2, max2))| {
    (min1..=max1).contains(&min2)
        || (min1..=max1).contains(&max2)
        || (min2..=max2).contains(&min1)
        || (min2..=max2).contains(&max1)
})
```

Simple, right?
