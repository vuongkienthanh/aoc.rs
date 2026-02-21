use crate::parsing::{Point, parse_input};
use crate::scanner_variations;
use fxhash::FxHashMap;

pub fn process(_input: &str) -> usize {
    let mut input = parse_input(_input);
    input.iter_mut().for_each(|v| v.sort_unstable());
    // assume left scanner is (0,0,0), this return right scanner coord
    let mut distances: FxHashMap<(usize, usize), Point> = FxHashMap::default();
    for i in 0..input.len() - 1 {
        let variations = scanner_variations(&input[0]);
        'j: for j in i + 1..input.len() {
            for v in &variations {
                let diff_map: FxHashMap<Point, usize> = FxHashMap::default();
                for diff in input
                    .get_mut(j)
                    .unwrap()
                    .iter()
                    .zip(v)
                    .map(|((a, b, c), (x, y, z))| (x - a, y - b, z - c))
                {
                    *diff_map.entry(diff).or_default() += 1;
                }
                for (diff, count) in diff_map {
                    if count >=12 {
                    distances.entry((i,j))

                        continue 'j

                    }
                }
            }
        }
    }

    0
}
