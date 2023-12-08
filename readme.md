# Advent of Code 2023

Another year of learning rust! Again! WHOOOOO!
Coding in another language that I don't normally use is always pretty fun!
This year, my aim is to better learn about ownership!

## Running code

```sh
cargo run day<n>::part<n>::tests::day<n>part<n>
# IE: day3::part1::tests::day3part1
```

## Learnings

### Mutable references

[StackOverflow](https://stackoverflow.com/questions/40921712/rust-mutable-value-vs-mutable-reference)
Notice the mut is on both sides

```rust
fn main() {
    let mut x = &mut 5;
    do_work(x);
    println!("{}", x);
}

fn do_work(n: &mut u32) {
    *n += 5;
}
// this is 10
```
