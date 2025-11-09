# An Advent Of Code template generator and solver in Rust

## Prepare
Edit `.env` in root folder, provide the following environment variables:
- `AOC_session`
- `AOC_year`
- `AOC_num_of_day`

You can look up session in your browser: `F12` -> `Application tab` -> `Cookies`

```sh
AOC_session=my_secret_session_key
AOC_year=2025
AOC_num_of_day=12
```

You also need [`bacon`](https://dystroy.org/bacon/) to help you coding and running jobs.
```sh
cargo install --locked bacon
```

## Installation
- Clone repo and build
```sh
cargo build --release
```

## Usage
I made simple shortcuts to generate workspace and fetch input
```sh
chmod u+x ./gen ./fetch

./gen
## generate template
./fetch 1 
## fetch day 1 input
```

## Solve
- `cd 2025/day01` for example. Edit `src/part1.rs` and `src/part2.rs` correspondingly.
- Use `bacon` to help coding and running jobs ( mostly used key `1` `2` `3` `4`).

# Recommended crates

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
