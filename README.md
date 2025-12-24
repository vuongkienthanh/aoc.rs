# An Advent Of Code template generator and solver in Rust

## Prepare
Edit `.env` in root folder, provide the following environment variables:
- `AOC_session`
- `AOC_year`

You can look up session in your browser: `F12` -> `Application tab` -> `Cookies`

```sh
AOC_session=my_secret_session_key
AOC_year=2025
```

You also need [`bacon`](https://dystroy.org/bacon/) and `bat` to help you coding and running jobs.
```sh
cargo install --locked bacon
sudo apt install bat
```

## Installation
- Clone repo and build
```sh
cargo build --release
```

## Usage
I made simple shortcuts to generate workspace and fetch input. First, you need to change permission
```sh
chmod u+x ./gen ./fetch
```

Then use these two commands:
```sh
./gen
## generate template
./fetch 1 
## fetch day 1 input
./fetch all 
## fetch all input for that year
```

## Solve
- `cd 2025/day01` for example. Edit `src/part1.rs` and `src/part2.rs` correspondingly.
- Use `bacon` to help coding and running jobs ( mostly used key `1` `2` `3` `4`).
