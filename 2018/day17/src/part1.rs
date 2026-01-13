use crate::parsing::parse_input;
use crate::{build_map, display_map, find_base};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let (mut map, min_x) = build_map(input);
    display_map(&map);
    let mut springs = vec![(500 - min_x, 0)];
    while !springs.is_empty() {
        let mut new_springs = vec![];
        for spring in springs {
            if let Some(base) = find_base(spring, &mut map) {

            }
        }
        springs = new_springs;
    }
    //     let mut new_springs = vec![];
    //     for spring in springs {
    //         if let Some((y, (mut left, mut right))) = find_base(spring, &bases) {
    //             println!(
    //                 "at spring {spring:?}: found y={y} ({left},{right}), so add straight flow ={}",
    //                 y - spring.1 - 1
    //             );
    //             ans += y - spring.1 - 1;
    //             for y in (spring.1..y).rev() {
    //                 match find_wall(spring, y, (left, right), &walls) {
    //                     (Some(left_x), Some(right_x)) => {
    //                         (left, right) = (left_x, right_x);
    //                         println!(
    //                             "at y={y}, found ({left_x},{right_x}) so add {}",
    //                             right - left - 2
    //                         );
    //                         ans += right - left - 2;
    //                     }
    //                     (Some(left_x), None) => {
    //                         left = left_x;
    //                         println!(
    //                             "at y={y}, found left={left_x}, new_spring at ({},{y}) so add {}",
    //                             right + 1,
    //                             right - left,
    //                         );
    //                         ans += right - left;
    //                         if seen.insert((right + 1, y)) {
    //                             new_springs.push((right + 1, y));
    //                         }
    //                         break;
    //                     }
    //                     (None, Some(right_x)) => {
    //                         right = right_x;
    //                         println!(
    //                             "at y={y}, found right={right_x}, new_spring at ({},{y}) so add {}",
    //                             left - 1,
    //                             right - left,
    //                         );
    //                         ans += right - left;
    //                         if seen.insert((left - 1, y)) {
    //                             new_springs.push((left - 1, y));
    //                         }
    //                         break;
    //                     }
    //                     (None, None) => {
    //                         println!(
    //                             "at y={y}, found no wall, new_spring at ({},{y}) and ({},{y}) so add {}",
    //                             left - 1,
    //                             right + 1,
    //                             right - left + 2,
    //                         );
    //                         ans += right - left + 2;
    //                         if seen.insert((right + 1, y)) {
    //                             new_springs.push((right + 1, y));
    //                         }
    //                         if seen.insert((left - 1, y)) {
    //                             new_springs.push((left - 1, y));
    //                         }
    //                         break;
    //                     }
    //                 }
    //             }
    //         } else {
    //             println!(
    //                 "at spring {spring:?}: not found y, so add straight flow ={}",
    //                 max_y - spring.1
    //             );
    //             ans += max_y - spring.1;
    //         }
    //     }
    //     springs = new_springs;
    //     if let Some((_, min_y)) = springs.iter().min_by_key(|(x, y)| *y) {
    //         bases.retain(|y, _| y >= min_y);
    //     }
    //     println!("new_springs={springs:?}");
    // }

    display_map(&map);
    let ans: usize = map
        .into_iter()
        .map(|line| line.into_iter().filter(|x| *x == '|' || *x == '~').count())
        .sum();
    println!("ans = {ans}");

    todo!("part1");
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
