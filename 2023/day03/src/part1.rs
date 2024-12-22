use crate::get_adj_for_number;
pub fn process(_input: &str) -> usize {
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
        assert_eq!(process(input), 4361);
    }
}
