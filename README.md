# An Advent Of Code template generator and solver in Rust

## Prepare
- Edit `.env` in root folder, provide `AOC_session` and `AOC_year`. You can look up session in your browser, Network tab, reload page and look for Cookie session in request object.
```sh
AOC_session=blahblah
AOC_year=2023
```

## Command s
- Build
```sh
cargo build --release
```

- Shortcuts: I made simple shortcuts to generate workspace and fetch input
```sh
chmod +x ./gen
chmod +x ./fetch
./gen
./fetch 1
```

## Solve
- `cd 2023/day01` for example. Edit `src/part1.rs` and `src/part2.rs` correspondingly.
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
cargo test 1
cargo test 2
```

To solve:
```sh
cargo run --bin p1
cargo run --bin p2
```

# Recommended crates & tools

## Parsing
- [nom](https://crates.io/crates/nom) - easy parsing input
- [nom_locate](https://crates.io/crates/nom_locate) - nom with location


## Multi processing
- [rayon](https://crates.io/crates/rayon) - parallel processing
- [crossbeam](https://crates.io/crates/crossbeam) - concurrent programming

## Graph
- [petgraph](https://crates.io/crates/petgraph) - graph data structure
- [pathfinding](https://crates.io/crates/pathfinding) - generic path finding algos
- [graphviz](https://graphviz.org/documentation/) - graph visualization

## Testing
- [rstest](https://crates.io/crates/rstest) - fixture and table-based testing

## Ultilities
- [itertools](https://crates.io/crates/itertools) - versatile iterator
- [grid](https://crates.io/crates/grid) - faster than Vec<Vec<T>>
