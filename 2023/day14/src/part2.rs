use std::{collections::HashMap, ops::Rem};

type Platform = Vec<Vec<char>>;

fn tilt_north(platform: &mut Platform) {
    let rows = platform.len();
    let cols = platform.first().unwrap().len();
    for row in 1..rows {
        for col in 0..cols {
            let cell = platform.get(row).unwrap().get(col).unwrap();
            if cell == &'O' {
                let mut i = row - 1;
                let nearest_unmovable = loop {
                    if platform.get(i).unwrap().get(col).unwrap() != &'.' {
                        break Some(i);
                    } else if i == 0 {
                        break None;
                    } else {
                        i -= 1
                    }
                };
                //begin move
                *platform.get_mut(row).unwrap().get_mut(col).unwrap() = '.';
                if let Some(r) = nearest_unmovable {
                    *platform.get_mut(r + 1).unwrap().get_mut(col).unwrap() = 'O';
                } else {
                    *platform.get_mut(0).unwrap().get_mut(col).unwrap() = 'O';
                }
            }
        }
    }
}

fn tilt_west(platform: &mut Platform) {
    let rows = platform.len();
    let cols = platform.first().unwrap().len();
    for col in 1..cols {
        for row in 0..rows {
            let cell = platform.get(row).unwrap().get(col).unwrap();
            if cell == &'O' {
                let mut i = col - 1;
                let nearest_unmovable = loop {
                    if platform.get(row).unwrap().get(i).unwrap() != &'.' {
                        break Some(i);
                    } else if i == 0 {
                        break None;
                    } else {
                        i -= 1
                    }
                };
                //begin move
                *platform.get_mut(row).unwrap().get_mut(col).unwrap() = '.';
                if let Some(c) = nearest_unmovable {
                    *platform.get_mut(row).unwrap().get_mut(c + 1).unwrap() = 'O';
                } else {
                    *platform.get_mut(row).unwrap().get_mut(0).unwrap() = 'O';
                }
            }
        }
    }
}

fn tilt_south(platform: &mut Platform) {
    let rows = platform.len();
    let cols = platform.first().unwrap().len();
    for row in (0..rows - 1).rev() {
        for col in 0..cols {
            let cell = platform.get(row).unwrap().get(col).unwrap();
            if cell == &'O' {
                let mut i = row + 1;
                let nearest_unmovable = loop {
                    if platform.get(i).unwrap().get(col).unwrap() != &'.' {
                        break Some(i);
                    } else if i == rows - 1 {
                        break None;
                    } else {
                        i += 1
                    }
                };
                //begin move
                *platform.get_mut(row).unwrap().get_mut(col).unwrap() = '.';
                if let Some(r) = nearest_unmovable {
                    *platform.get_mut(r - 1).unwrap().get_mut(col).unwrap() = 'O';
                } else {
                    *platform.get_mut(rows - 1).unwrap().get_mut(col).unwrap() = 'O';
                }
            }
        }
    }
}

fn tilt_east(platform: &mut Platform) {
    let rows = platform.len();
    let cols = platform.first().unwrap().len();
    for row in 0..rows {
        for col in (0..cols - 1).rev() {
            let cell = platform.get(row).unwrap().get(col).unwrap();
            if cell == &'O' {
                let mut i = col + 1;
                let nearest_unmovable = loop {
                    if platform.get(row).unwrap().get(i).unwrap() != &'.' {
                        break Some(i);
                    } else if i == cols - 1 {
                        break None;
                    } else {
                        i += 1
                    }
                };
                //begin move
                *platform.get_mut(row).unwrap().get_mut(col).unwrap() = '.';
                if let Some(c) = nearest_unmovable {
                    *platform.get_mut(row).unwrap().get_mut(c - 1).unwrap() = 'O';
                } else {
                    *platform.get_mut(row).unwrap().get_mut(cols - 1).unwrap() = 'O';
                }
            }
        }
    }
}

fn cycle_once(platform: &mut Platform) {
    tilt_north(platform);
    tilt_west(platform);
    tilt_south(platform);
    tilt_east(platform);
}

fn cycle_once_with_cache(
    platform: &mut Platform,
    cache: &mut HashMap<Platform, usize>,
    cycle: usize,
) -> Option<usize> {
    if cache.contains_key(platform) {
        cache.get(platform).copied()
    } else {
        let origin = platform.clone();
        cycle_once(platform);
        cache.insert(origin, cycle);
        None
    }
}

pub fn process(_input: &str) -> usize {
    let mut platform = _input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let rows = platform.first().unwrap().len();
    let mut cache = HashMap::new();
    let mut distance_at_start_loop = 0;
    let mut loop_len = 0;

    for i in 0..1_000_000_000 {
        if let Some(idx) = cycle_once_with_cache(&mut platform, &mut cache, i) {
            distance_at_start_loop = idx;
            loop_len = i - idx;
            break;
        }
    }

    let so_far = distance_at_start_loop + loop_len;
    let rest = 1_000_000_000 - so_far;
    let after_modulo = rest.rem(loop_len);

    for _ in 0..after_modulo {
        cycle_once(&mut platform);
    }

    (0..rows)
        .map(|i| {
            let score = rows - i;
            platform
                .get(i)
                .unwrap()
                .into_iter()
                .filter(|c| c == &&'O')
                .count()
                * score
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
        assert_eq!(process(input), 64);
    }
}
