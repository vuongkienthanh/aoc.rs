pub mod part1;
pub mod part2;

use std::collections::HashSet;
use itertools::Itertools;
pub fn get_adj_for_number(
    line_index: usize,
    start: usize,
    end: usize,
    _input: &str,
) -> Vec<(usize, usize)> {
    let row_max = _input.lines().count() - 1;
    let col_max = _input.lines().next().unwrap().len() - 1;

    let mut rows = HashSet::from([line_index]);
    rows.insert(line_index.saturating_sub(1));
    rows.insert((line_index + 1).min(row_max));

    let mut cols: HashSet<usize> = HashSet::from_iter(start..=end);
    cols.insert(start.saturating_sub(1));
    cols.insert((end + 1).min(col_max));

    rows.iter()
        .cartesian_product(cols.iter())
        .filter_map(|(a, b)| {
            if *a == line_index && *b >= start && *b <= end {
                None
            } else {
                Some((*a, *b))
            }
        })
        .collect_vec()
}
