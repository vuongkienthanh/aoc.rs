use std::collections::VecDeque;
use std::ops::Range;

/// return (origin, modified)
fn overlap(
    lhs: &Range<isize>,
    rhs: &Range<isize>,
    diff: isize,
) -> (Vec<Range<isize>>, Vec<Range<isize>>) {
    // rhs contains whole lhs
    if rhs.start <= lhs.start && rhs.end >= lhs.end {
        (
            vec![],
            vec![Range {
                start: lhs.start + diff,
                end: lhs.end + diff,
            }],
        )
    // rhs doesn't overlap lhs
    } else if rhs.end <= lhs.start || rhs.start >= lhs.end {
        (
            vec![Range {
                start: lhs.start,
                end: lhs.end,
            }],
            vec![],
        )
    // small rhs on the left; same/longer start
    } else if rhs.start <= lhs.start && rhs.end < lhs.end {
        (
            vec![Range {
                start: rhs.end,
                end: lhs.end,
            }],
            vec![Range {
                start: lhs.start + diff,
                end: rhs.end + diff,
            }],
        )
    // small rhs on the right; same/longer end
    } else if rhs.start > lhs.start && rhs.end >= lhs.end {
        (
            vec![Range {
                start: lhs.start,
                end: rhs.start,
            }],
            vec![Range {
                start: rhs.start + diff,
                end: lhs.end + diff,
            }],
        )
    // small rhs in the middle
    } else if rhs.start > lhs.start && rhs.end < lhs.end {
        (
            vec![
                Range {
                    start: lhs.start,
                    end: rhs.start,
                },
                Range {
                    start: rhs.end,
                    end: lhs.end,
                },
            ],
            vec![Range {
                start: rhs.start + diff,
                end: rhs.end + diff,
            }],
        )
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
        // dbg!("new group");
        for _ in 0..seed_map.len() {
            let seed_range = seed_map.pop_front().unwrap();
            // dbg!(&seed_range);
            //
        }
    }
    seed_map.iter().map(|x| x.start).min().unwrap();
    0
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
