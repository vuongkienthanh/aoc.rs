# Advent of code templating in Rust

### Basic usage
Edit `.env` in root folder with `AOC_session` and `AOC_year`.

You can look up session in your chrome's F12, Network tab, reload AOC input page and look for Cookie in request object.
```sh
AOC_session=blahblah
AOC_year=2023
```

Generate workspace
```sh
# /aoc/2023/day01 and so on
cargo run --release -- gen 
```

Fetch input
```sh
# /aoc/2023/day01/src/input.txt and so on
cargo run --release -- fetch 1 # puzzle day
```

### Solve
`cd` into `2023/day01` for example.

Edit `src/part1` and `src/part2` correspondingly.
```sh
cargo test --lib
```

```sh
cargo run --bin p1
cargo run --bin p2
```

# Recommended crates
- [nom](https://crates.io/crates/nom) - easy parsing input
- [itertools](https://crates.io/crates/itertools) - very useful to iterate
