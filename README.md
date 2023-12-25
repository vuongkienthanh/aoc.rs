# An Advent Of Code template and solver in Rust

### Template
Edit `.env` in root folder with `AOC_session` and `AOC_year`.

You can look up session in your chrome's F12, Network tab, reload AOC input page and look for Cookie in request object.
```sh
AOC_session=blahblah
AOC_year=2023
```

Generate workspace
```sh
# generate /aoc.rs/2023/day01 and so on
cargo run --release -- gen 
```

Fetch input
```sh
# download into /aoc/2023/day01/src/input.txt and so on
cargo run --release -- fetch 1 # puzzle day
```

Shortcuts
```sh
cargo build --release
chmod +x ./fetch
./fetch 1
```

### Solve
Go inside `2023/day01` for example.
```sh
cd 2023/day01
```

Edit `src/part1.rs` and `src/part2.rs` correspondingly.
```rust
pub fn process(_input: &str) -> usize {
    todo!("part1") // edit your solution here
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#""#; // put example input here
        assert_eq!(process(input), 0); // put example answer here
    }
}
```

To run tests:
```sh
cargo test --lib part1
cargo test --lib part2
```

To solve:
```sh
cargo run --bin p1
cargo run --bin p2
```

# Recommended crates
- [nom](https://crates.io/crates/nom) - easy parsing input
- [itertools](https://crates.io/crates/itertools) - versatile iterator
- [rayon](https://crates.io/crates/rayon) - parallel processing
