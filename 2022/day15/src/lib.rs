pub mod parsing;
pub mod part1;
pub mod part2;
use aoc_helper::range::merge;
use parsing::Point;

pub fn range_at(map: &[(Point, usize)], at: isize) -> Vec<(isize, isize)> {
    let mut ans = vec![];
    map.iter().for_each(|((s0, s1), radius)| {
        let diff = at.abs_diff(*s1);
        if let Some(missing) = radius.checked_sub(diff) {
            ans.push((
                s0.checked_sub_unsigned(missing).unwrap(),
                s0.checked_add_unsigned(missing).unwrap(),
            ))
        }
    });
    merge(ans)
}

pub fn radius_map(input: Vec<(Point, Point)>) -> Vec<(Point, usize)> {
    input
        .into_iter()
        .map(|((s0, s1), (b0, b1))| {
            let radius = b0.abs_diff(s0) + b1.abs_diff(s1);
            ((s0, s1), radius)
        })
        .collect()
}
