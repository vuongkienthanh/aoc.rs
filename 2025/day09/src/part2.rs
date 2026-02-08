use crate::calc_area;
use crate::parsing::parse_input;
use aoc_helper::range::merge;
use std::collections::BTreeMap;

/// from col left to col right
type Range = (usize, usize);
type Point = (usize, usize);

fn range_contains(range: Range, col: usize) -> bool {
    col >= range.0 && col <= range.1
}

pub fn process(_input: &str) -> usize {
    let mut row_state_map: BTreeMap<usize, (Range, Vec<Range>, bool)> = parse_input(_input)
        .into_iter()
        .map(|(k, v)| (k, (v, vec![v], false)))
        .collect();

    let mut max_area = 0;

    while let Some((top_row, ((top_col0, top_col1), ranges, is_blocker))) =
        row_state_map.pop_first()
    {
        if is_blocker {
            continue;
        }

        let mut ranges = merge(ranges);
        ranges.retain(|(a, b)| a <= &top_col0 && b >= &top_col1);
        assert_eq!(ranges.len(), 1);
        let usable_range = ranges.pop().unwrap();

        // each subsequent row mutates these 2 ranges
        let mut left_range = Some((top_col0, usable_range.1));
        let mut right_range = Some((usable_range.0, top_col1));

        for (bottom_row, ((bottom_col0, bottom_col1), ranges, is_blocker)) in
            row_state_map.iter_mut()
        {
            // check top_left ranges
            if let Some((top_col0, top_col1)) = left_range {
                let top_left: Point = (top_row, top_col0);

                // prioritize bottom_right
                if range_contains((top_col0, top_col1), *bottom_col1) {
                    let bottom_right: Point = (*bottom_row, *bottom_col1);
                    let area = calc_area(top_left, bottom_right);
                    max_area = max_area.max(area);
                } else if range_contains((top_col0, top_col1), *bottom_col0) {
                    let bottom_left: Point = (*bottom_row, *bottom_col0);
                    let area = calc_area(top_left, bottom_left);
                    max_area = max_area.max(area);
                }
                // check is blocker and trim range
                if top_col0 >= *bottom_col0 && top_col0 < *bottom_col1 {
                    left_range = None;
                    *is_blocker = true;
                } else if top_col0 < *bottom_col0 && top_col1 > *bottom_col0 {
                    left_range = Some((top_col0, *bottom_col0));
                    *is_blocker = true;
                }
            }

            // check top_right ranges
            if let Some((top_col0, top_col1)) = right_range {
                let top_right: Point = (top_row, top_col1);
                // prioritize bottom_left
                if range_contains((top_col0, top_col1), *bottom_col0) {
                    let bottom_left: Point = (*bottom_row, *bottom_col0);
                    let area = calc_area(top_right, bottom_left);
                    max_area = max_area.max(area);
                } else if range_contains((top_col0, top_col1), *bottom_col1) {
                    let bottom_right: Point = (*bottom_row, *bottom_col1);
                    let area = calc_area(top_right, bottom_right);
                    max_area = max_area.max(area);
                }
                // check is blocker and trim range
                if top_col1 <= *bottom_col1 && top_col1 > *bottom_col0 {
                    right_range = None;
                    *is_blocker = true;
                } else if top_col1 > *bottom_col1 && top_col0 < *bottom_col1 {
                    right_range = Some((*bottom_col1, top_col1));
                    *is_blocker = true;
                }
            }

            // update the usable ranges on this bottom row
            if !*is_blocker {
                if let Some(r) = left_range {
                    ranges.push(r);
                }
                if let Some(r) = right_range {
                    ranges.push(r);
                }
            }
            // finish the top row when no viable left right ranges
            if left_range.is_none() && right_range.is_none() {
                break;
            }
        }
    }

    max_area
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 24);
    }
}
