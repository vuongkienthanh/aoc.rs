# An Advent Of Code template generator and solver in Rust

## Prepare
Edit `.env` in root folder, provide the following environment variables:
    - `AOC_session`
    - `AOC_year`
    - `AOC_num_of_day`

You can look up session in your browser:

`F12` -> `Application tab` -> `Cookies`

```sh
AOC_session=my_secret_session_key
AOC_year=2025
AOC_num_of_day=12
```

## Command
- Build
```sh
cargo build --release
```

- Shortcuts: I made simple shortcuts to generate workspace and fetch input
```sh
chmod +x ./gen ./fetch

./gen
## generate template
./fetch 1 
## fetch day 1 input
```

## Solve
- `cd 2025/day01` for example. Edit `src/part1.rs` and `src/part2.rs` correspondingly.
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

To run all tests:
```sh
cargo test
```

To solve:
```sh
cargo run --bin p1
cargo run --bin p2
```

# Recommended crates & tools

## Parsing
- [nom](https://crates.io/crates/nom) - combinator parser
- [nom_locate](https://crates.io/crates/nom_locate) - nom with location

## Graph
- [petgraph](https://crates.io/crates/petgraph) - graph data structure
- [pathfinding](https://crates.io/crates/pathfinding) - generic path finding algos
- [graphviz](https://graphviz.org/documentation/) - graph visualization

## Testing
- [rstest](https://crates.io/crates/rstest)

## Ultilities
- [itertools](https://crates.io/crates/itertools)
- [grid](https://crates.io/crates/grid)
