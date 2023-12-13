use std::collections::VecDeque;
use std::ops::Add;
use std::ops::Range;

struct OverlapResult<Idx> {
    new_origins: Vec<Range<Idx>>,
    modified: Option<Range<Idx>>,
}

fn overlap<Idx>(lhs: &Range<Idx>, rhs: &Range<Idx>, diff: &Idx) -> OverlapResult<Idx>
where
    Idx: Ord + PartialOrd + Eq + PartialEq + Add<Output = Idx> + Copy + Clone,
{
    // rhs contains whole lhs
    if rhs.start <= lhs.start && rhs.end >= lhs.end {
        OverlapResult {
            new_origins: vec![],
            modified: Some(Range {
                start: lhs.start + *diff,
                end: lhs.end + *diff,
            }),
        }
    // rhs doesn't overlap lhs
    } else if rhs.end <= lhs.start || rhs.start >= lhs.end {
        OverlapResult {
            new_origins: vec![Range {
                start: lhs.start,
                end: lhs.end,
            }],
            modified: None,
        }
    // small rhs on the left; same/longer start
    } else if rhs.start <= lhs.start && rhs.end < lhs.end {
        OverlapResult {
            new_origins: vec![Range {
                start: rhs.end,
                end: lhs.end,
            }],
            modified: Some(Range {
                start: lhs.start + *diff,
                end: rhs.end + *diff,
            }),
        }
    // small rhs on the right; same/longer end
    } else if rhs.start > lhs.start && rhs.end >= lhs.end {
        OverlapResult {
            new_origins: vec![Range {
                start: lhs.start,
                end: rhs.start,
            }],
            modified: Some(Range {
                start: rhs.start + *diff,
                end: lhs.end + *diff,
            }),
        }
    // small rhs in the middle
    } else if rhs.start > lhs.start && rhs.end < lhs.end {
        OverlapResult {
            new_origins: vec![
                Range {
                    start: lhs.start,
                    end: rhs.start,
                },
                Range {
                    start: rhs.end,
                    end: lhs.end,
                },
            ],
            modified: Some(Range {
                start: rhs.start + *diff,
                end: rhs.end + *diff,
            }),
        }
    } else {
        unreachable!()
    }
}
pub fn process(_input: &str) -> isize {
    let mut groups = _input.split("\n\n");
    let seed_row = groups
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let mut seed_map = (0..seed_row.len())
        .step_by(2)
        .map(|i| Range {
            start: *seed_row.get(i).unwrap(),
            end: *seed_row.get(i).unwrap() + *seed_row.get(i + 1).unwrap(),
        })
        .collect::<VecDeque<Range<isize>>>();

    let parsed_groups = groups
        .map(|group| {
            group
                .lines()
                .skip(1)
                .map(|line| {
                    let mut linesplit = line.split_ascii_whitespace();
                    let dst_range_start = linesplit.next().unwrap().parse::<isize>().unwrap();
                    let src_range_start = linesplit.next().unwrap().parse::<isize>().unwrap();
                    let range_length = linesplit.next().unwrap().parse::<isize>().unwrap();

                    let rhs = Range {
                        start: src_range_start,
                        end: src_range_start + range_length,
                    };
                    let diff = dst_range_start - src_range_start;
                    (rhs, diff)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for group in parsed_groups.iter() {
        // use len() to keep track of seed_map range count at the start of each group
        // because seed_map could be extended with a modified range at the end of each line
        for _ in 0..seed_map.len() {
            let seed_range = seed_map.pop_front().unwrap();

            let mut origin_ranges: Vec<Range<isize>> = vec![seed_range];

            // for each line
            for (rhs, diff) in group {
                // placeholder to replace origin
                let mut new_origin_ranges: Vec<Range<isize>> = vec![];
                // origin_ranges may have 1 or many
                while let Some(lhs) = origin_ranges.pop() {
                    // overlap() gives new origin ranges and Option<modified>
                    // overlap() may give back origin because no src at all -> stay the same
                    let result = overlap(&lhs, rhs, diff);

                    new_origin_ranges.extend(result.new_origins.into_iter());

                    // extend seed_map with modified
                    if let Some(modified) = result.modified {
                        seed_map.push_back(modified);
                    }
                }
                // at the end of line
                // origin is replaced by new origin
                // prepared to overlap() next line
                origin_ranges = new_origin_ranges;
            }
            // after all lines ( a group )
            // there should be zero or some new origin ranges that is no src
            // they shoule be merged back into seed_map
            seed_map.extend(origin_ranges.into_iter());
        // next seed_range, but never longer than seed_map length at the beginning of group
        }
    }
    seed_map.iter().map(|x| x.start).min().unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
        assert_eq!(process(input), 46);
    }
}
