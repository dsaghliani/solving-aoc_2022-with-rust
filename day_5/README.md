## Advent of Code 2022 - Day 05

[See the entire, unedited problem on the official website.](https://adventofcode.com/2022/day/5)

We're given rows of crates, stacked in columns, and instructions for a crane to follow.

```
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
```

The first part represents the stacks of crates, and the second is the instructions. You can read them as, "Move **3 crates** from **Stack 1** to **Stack 3**."

### Part 1

**Problem:** After all the instructions have been followed, determine which crates will end up on top of each stack. The crates are moved **one at a time**.

The expected result for the sample input above is "CMZ":

```
        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3
```

The problem can be broken down into three components:

1. Parse the crates into some usable format.
2. Parse the instructions.
3. Apply the instructions and calculate the output (the topmost crates).

#### Parsing the Crates.

I chose to store the configuration of the crates as a `HashMap`, where the position of a stack—in other words, its column—is the key and the crates themselves are a vector of characters.

```rs
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

            if let Some(chars) = crates.get_mut(&col) {
                chars.insert(0, char);
            } else {
                crates.insert(col, vec![char]);
            }

            crates
        })
}
```

Let's break that down.

1. The `take_while()` makes sure the iteration ends where it should.
2. Then we break up every line into groups of four using `array_chunks()`, a delightful little function from Nightly Rust.
3. Only then do we enumerate the elements. Had we done that before the chunking, we'd be counting characters. With this, we're counting chunks, and that tells us which column we're on.
4. We pattern match on the column and the array of 4 characters. The array will either be all whitespace or come in the form of "[_] ", so we check the second character.
5. Finally, we use `fold()` to continue the chain. It could be replaced with a procedural `for` loop, if that's your preference. Either way, we add the character to the beginning of the vector `stack`, because the iteration starts from the top, not the bottom. It'd be more efficient to just reverse the stacks once after the iteration or use `VecDeque`, and if the input was great enough I would, but as it is, I opted for simplicity.

Take note of `chain(iter::once(' '))`. We're chunking in fours because the first `n - 1` stacks (where `n` is the amount of columns) are divisible into groups of "[_] ", but the rightmost stacks do not have trailing whitespace and are three characters long, so `array_chunks()` would leave them out. This fixes that.

#### Figuring Out Which Crates End Up On Top

While we're at it, let's also write a function that gets the topmost crates and builds the output string.

```rs
fn get_topmost_crates(crates: HashMap<usize, Vec<char>>) -> String {
    crates
        .into_iter()
        .sorted()
        .filter_map(|(_col, mut stack)| stack.pop())
        .collect()
}
```

Worthy of attention here is the `sorted()` part. That's a convenience function from `itertools` that takes an iterator and converts it to a sorted one.

It's impossible to sort a collection during iteration. You need to allocate the space for all the elements first. `sorted()` doesn't break that law; it allocates a vector behind the scenes, sorts it, and returns an iterator to it, letting us keep the builder pattern going.

#### Parsing the Instructions

This part is easy. Every instruction is exactly one line long and contains three numbers. All we need to do is extract them.

```rs
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
```

`filter_map()` takes a closure that returns `Option`. It discards any `None` values and automatically unwraps the `Some()` ones. However, `parse()` returns a `Result`, so we call `ok()` to convert it into `Option`.

Once again, we're using `collect_tuple()` from `itertools` to make things easier. If you dislike the use of `expect()`, it's possible to return `Result<Vec<...>, ...>`, like so:

```rs
fn parse_instructions(input: &str) -> Result<Vec<(usize, usize, usize)>, &'static str> {
    input
        .lines()
        .filter(|line| line.starts_with("move"))
        .map(|line| {
            line.split(' ')
                .filter_map(|str| str.parse::<usize>().ok())
                .collect_tuple()
                .ok_or("Every instruction should contain 3 numbers.")
        })
        .collect::<Result<Vec<_>, _>>()
}
```

In a real program, leaving error-handling to the caller may be wise, but I think `expect()` is better suited here, since we'd be unwrapping the return value anyway. I'll go with the initial version.

#### Putting It All Together

Now that we've written all the helper functions, it's time to assemble them. The only logic left is following the instructions.

```rs
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
```

Two things stand out:

- `stack.split_off()`
- `stack.extend()`

[`Vec::split_off()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_off) receives an index as an argument and takes all the elements from there on, returning them as a separate vector. The original vector is mutated and loses the elements. Because we're taking from the top, we count from the end, subtracting `amount` from the total length.

There's a problem with that, though. Imagine we want to take 2 crates, and the stack looks as follows:

```
C
B
A
```

Since we're taking them one at a time, we'd take C and then put B on top, leaving us with:

```
  B
A C
```

However, `split_off` grabs the elements all at once, giving us this:

```
  C
A B
```

That's why we use [`Extend::extend()`](https://doc.rust-lang.org/std/iter/trait.Extend.html#tymethod.extend) (`Extend` being a trait `Vec` implements) to add the new crates in reverse order.

### Part 2

**Problem:** Turns out, the crane works differently than expected. Instead of grabbing crates one at a time, it can handle many at once.

The expected result is now "MCD":

```
        [D]
        [N]
        [Z]
[M] [C] [P]
 1   2   3
```

What this means is that the finessing we did at the end of Part 1, reversing the order of the new crates, is no longer necessary.

```rs
pub fn process_part_2(input: &str) -> String {
    let mut crates = parse_crates(input);
    let instructions = parse_instructions(input);

    for (amount, from, to) in instructions {
-       let popped_crates = crates.get_mut(&from).map_or_else(
+       let mut popped_crates = crates.get_mut(&from).map_or_else(
            || panic!("Instruction tried to take from a stack that doesn't exist."),
            |stack| stack.split_off(stack.len() - amount),
        );

        crates.get_mut(&to).map_or_else(
            || panic!("Instruction tried to add to a stack that doesn't exist."),
-           |stack| stack.extend(popped_crates.into_iter().rev()),
+           |stack| stack.append(&mut popped_crates),
        );
    }

    get_topmost_crates(crates)
}
```

Instead of `extend()`, which takes an iterator, we use [`Vec::append()`](https://doc.rust-lang.org/std/vec/struct.Vec.html?search=extend#method.append) and pass in a mutable reference to the vector. (Which is why it's declared mutable.)

That's it!