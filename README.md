# Advent of code solver in Rust
---

### Basic usage

Edit .env in root folder with `AOC_session` and `AOC_year`.
You can look up session in your chrome's F12, Network tab, reload AOC input page and look for Cookie in request object.

```sh
AOC_session=blahblah
AOC_year=2023
```

Generate workspace
```sh
# /aoc/2023/day01 and so on
cargo run -- gen 
```

Fetch input
```sh
# /aoc/2023/day01/input.txt and so on
cargo run -- fetch 1  # puzzle day
```

### Solver
`cd` into for example `/aoc/2023/day01`.
Edit corresponding main.rs
```sh
cargo test
cargo run
```
