pub mod part1;
pub mod part2;

use std::{cell::LazyCell, collections::HashMap};

type Coord = (isize, isize);
const DIRECTIONAL_KEYPAD: LazyCell<HashMap<char, Coord>> = LazyCell::new(|| {
    [" ^A", "<v>"]
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .map(move |(j, c)| (c, (j as isize, i as isize)))
        })
        .collect::<HashMap<char, Coord>>()
});
const NUMERIC_KEYPAD: LazyCell<HashMap<char, Coord>> = LazyCell::new(|| {
    ["789", "456", "123", " 0A"]
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .map(move |(j, c)| (c, (j as isize, i as isize)))
        })
        .collect::<HashMap<char, Coord>>()
});

fn key_path_cost(
    cache: &HashMap<(usize, char, char), usize>,
    robot: usize,
    key_start: char,
    key_end: char,
) -> usize {
    if robot == 0 {
        1
    } else {
        *cache
            .get(&(robot, key_start, key_end))
            .unwrap_or_else(|| panic!("invalid key '{} {} {}'", robot, key_start, key_end))
    }
}

fn keypresses_cost(
    cache: &HashMap<(usize, char, char), usize>,
    robot: usize,
    key_seq: &str,
) -> usize {
    format!("A{key_seq}") // robots always start at 'A'
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|v| (v[0], v[1]))
        .map(|(key_start, key_end)| key_path_cost(cache, robot, key_start, key_end))
        .sum()
}
fn cache_robot(
    cache: &mut HashMap<(usize, char, char), usize>,
    robot: usize,
    keypad: &HashMap<char, (isize, isize)>,
) {
    for (&key_start, &(x_start, y_start)) in keypad.iter() {
        for (&key_end, &(x_end, y_end)) in keypad.iter() {
            let horizontal_keys =
                if x_end > x_start { ">" } else { "<" }.repeat(x_end.abs_diff(x_start));
            let vertical_keys =
                if y_end < y_start { "^" } else { "v" }.repeat(y_end.abs_diff(y_start));

            let horizontal_key_seq = format!("{horizontal_keys}{vertical_keys}A");
            let vertical_key_seq = format!("{vertical_keys}{horizontal_keys}A");

            let min_horizontal = if (x_end, y_start) != keypad[&' '] {
                keypresses_cost(cache, robot - 1, &horizontal_key_seq)
            } else {
                usize::MAX
            };

            let min_vertical = if (x_start, y_end) != keypad[&' '] {
                keypresses_cost(cache, robot - 1, &vertical_key_seq)
            } else {
                usize::MAX
            };

            cache.insert(
                (robot, key_start, key_end),
                min_horizontal.min(min_vertical),
            );
        }
    }
}

fn cache_robots(n_robots: usize) -> HashMap<(usize, char, char), usize> {
    let mut cache: HashMap<(usize, char, char), usize> = HashMap::new();

    // build cache on top of each other
    for robot in 1..=n_robots {
        cache_robot(&mut cache, robot, &DIRECTIONAL_KEYPAD);
    }

    // last robot
    cache_robot(&mut cache, n_robots + 1, &NUMERIC_KEYPAD);

    cache
}

fn min_keypresses(code: &str, n_robots: usize) -> usize {
    let cache = cache_robots(n_robots);
    keypresses_cost(&cache, n_robots + 1, code)
}
