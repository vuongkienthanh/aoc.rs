use crate::parsing::parse_input;
use crate::{Point, build_map, display_map, fill_row, find_base};
use std::collections::BTreeSet;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let (mut map, min_x, min_y) = build_map(input);
    let mut seen = BTreeSet::new();
    let mut springs = vec![(500 - min_x + 1, 0)];
    while !springs.is_empty() {
        let mut new_springs: Vec<Point> = vec![];
        for spring in springs {
            if let Some(base) = find_base(spring, &mut map) {
                for y in (0..base).rev() {
                    let found_springs = fill_row(spring.clone(), y, &mut map);
                    if !found_springs.is_empty() {
                        for n_spring in found_springs {
                            if seen.insert(n_spring.clone()) {
                                new_springs.push(n_spring);
                            }
                        }
                        break;
                    }
                }
            }
        }
        springs = new_springs;
    }
    display_map(&map);
    map.into_iter()
        .skip(min_y)
        .map(|line| line.into_iter().filter(|x| *x == '|' || *x == '~').count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 57);
    }
}
