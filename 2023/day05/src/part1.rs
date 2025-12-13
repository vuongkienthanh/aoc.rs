pub fn process(_input: &str) -> usize {
    let mut groups = _input.split("\n\n");
    let mut seed_map = groups
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    for group in groups {
        let seed_map_before = seed_map.clone();

        for line in group.lines().skip(1) {
            let mut linesplit = line.split_ascii_whitespace();
            let dst_range_start = linesplit.next().unwrap().parse::<usize>().unwrap();
            let src_range_start = linesplit.next().unwrap().parse::<usize>().unwrap();
            let range_length = linesplit.next().unwrap().parse::<usize>().unwrap();

            for (idx, seed) in seed_map_before.iter().enumerate() {
                if (src_range_start..src_range_start + range_length).contains(seed) {
                    *seed_map.get_mut(idx).unwrap() += dst_range_start;
                    *seed_map.get_mut(idx).unwrap() -= src_range_start;
                }
            }
        }
    }
    seed_map.into_iter().min().unwrap()
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
        assert_eq!(process(input), 35);
    }
}
