use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_adj_for_number(
    line_index: usize,
    start: usize,
    end: usize,
    _input: &str,
) -> Vec<(usize, usize)> {
    let row_max = _input.lines().count() - 1;
    let col_max = _input.lines().next().unwrap().len() - 1;

    let mut rows = HashSet::from([line_index]);
    rows.insert(line_index.saturating_sub(1));
    rows.insert((line_index + 1).min(row_max));

    let mut cols: HashSet<usize> = HashSet::from_iter(start..=end);
    cols.insert(start.saturating_sub(1));
    cols.insert((end + 1).min(col_max));

    rows.iter()
        .cartesian_product(cols.iter())
        .filter_map(|(a, b)| {
            if *a == line_index && *b >= start && *b <= end {
                None
            } else {
                Some((*a, *b))
            }
        })
        .collect_vec()
}
fn is_part_number(line_index: usize, start: usize, end: usize, _input: &str) -> bool {
    let adjacent = get_adj_for_number(line_index, start, end, _input);
    for spot in adjacent {
        let spot_char = _input
            .lines()
            .nth(spot.0)
            .unwrap()
            .chars()
            .nth(spot.1)
            .unwrap();
        // adj to symbol
        if !(spot_char.is_ascii_digit() || spot_char == '.') {
            return true;
        }
    }
    false
}

fn part1(_input: &str) -> String {
    _input
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            let mut is_number = false;
            let mut start = 0;
            let mut line_sum = 0;

            for (i, c) in line.char_indices() {
                match is_number {
                    true if !c.is_ascii_digit() => {
                        is_number = false;
                        let end = i - 1;
                        if is_part_number(line_index, start, end, _input) {
                            line_sum += line.get(start..=end).unwrap().parse::<usize>().unwrap();
                        }
                    }
                    false if c.is_ascii_digit() => {
                        is_number = true;
                        start = i;
                    }
                    _ => (),
                }
            }
            if is_number {
                let end = line.len() - 1;
                if is_part_number(line_index, start, end, _input) {
                    line_sum += line.get(start..).unwrap().parse::<usize>().unwrap();
                }
            }
            line_sum
        })
        .sum::<usize>()
        .to_string()
}
fn is_part_number_and_adj_to_stars(
    line_index: usize,
    start: usize,
    end: usize,
    _input: &str,
) -> (bool, Vec<(usize, usize)>) {
    let mut flag = false;
    let mut stars = vec![];
    let adjacent = get_adj_for_number(line_index, start, end, _input);
    for spot in adjacent {
        let spot_char = _input
            .lines()
            .nth(spot.0)
            .unwrap()
            .chars()
            .nth(spot.1)
            .unwrap();
        // adj to star
        if spot_char == '*' {
            flag = true;
            stars.push((spot.0, spot.1));
        }
    }
    if stars.len() > 1 {
        println!("this will be printed out if there is a part number adjacent to 2 stars")
    };
    (flag, stars)
}
fn part2(_input: &str) -> String {
    let mut star_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    _input.lines().enumerate().for_each(|(line_index, line)| {
        let mut is_number = false;
        let mut start = 0;

        for (i, c) in line.char_indices() {
            match is_number {
                true if !c.is_ascii_digit() => {
                    is_number = false;
                    let end = i - 1;
                    let (is_part_number_with_stars, star_locs) =
                        is_part_number_and_adj_to_stars(line_index, start, end, _input);
                    if is_part_number_with_stars {
                        for loc in star_locs {
                            if star_map.entry(loc).or_default().len() < 2 {
                                let num = line.get(start..=end).unwrap().parse::<usize>().unwrap();
                                star_map.get_mut(&loc).unwrap().push(num);
                            }
                        }
                    }
                }
                false if c.is_ascii_digit() => {
                    is_number = true;
                    start = i;
                }
                _ => (),
            }
        }
        if is_number {
            let end = line.len() - 1;
            let (is_part_number_with_stars, star_locs) =
                is_part_number_and_adj_to_stars(line_index, start, end, _input);
            if is_part_number_with_stars {
                for loc in star_locs {
                    if star_map.entry(loc).or_default().len() < 2 {
                        let num = line.get(start..=end).unwrap().parse::<usize>().unwrap();
                        star_map.get_mut(&loc).unwrap().push(num);
                    }
                }
            }
        }
    });
    star_map
        .values()
        .filter_map(|v| {
            // is_gear
            if v.len() == 2 {
                Some(v.iter().product::<usize>())
            } else {
                None
            }
        })
        .sum::<usize>()
        .to_string()
}

fn main() {
    let input = include_str!("input.txt");
    // println!("{}", part1(input));
    println!("{}", part2(input));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_part1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(part1(input), "4361");
    }
    #[test]
    // #[ignore]
    fn test_part2() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(part2(input), "467835");
    }
}
