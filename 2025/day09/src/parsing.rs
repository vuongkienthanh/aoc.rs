use std::collections::BTreeMap;
use tinyvec::ArrayVec;

/// BTreeMap < row, vec of ele in row >
/// and also in col
/// the vec is sorted
/// notices that each row has only 2 ele, also each col has only 2 ele
pub fn parse_input(input: &str) -> BTreeMap<usize, (usize, usize)> {
    let mut row_map: BTreeMap<usize, ArrayVec<[usize; 2]>> = BTreeMap::new();
    for line in input.lines() {
        let mut i = line.split(',');
        let a = i.next().unwrap().parse::<usize>().unwrap();
        let b = i.next().unwrap().parse::<usize>().unwrap();
        row_map.entry(a).or_default().push(b);
    }
    for (_, v) in row_map.iter_mut() {
        v.sort_unstable();
    }
    row_map
        .into_iter()
        .map(|(k, v)| (k, (v[0], v[1])))
        .collect()
}
