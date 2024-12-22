use crate::get_adj_for_number;
use std::collections::HashMap;
pub fn process(_input: &str) -> usize {
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
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
        assert_eq!(process(input), 467835);
    }
}
