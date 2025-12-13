pub mod parsing;
pub mod part1;
pub mod part2;

use parsing::Point;

/// sorted pair of indices, instead of Point3D
pub fn to_sorted_pairs(points: &[Point]) -> Vec<(usize, usize)> {
    let mut ans = (0..points.len() - 1)
        .flat_map(|a| {
            (a + 1..points.len()).map(move |b| {
                let pa = points[a];
                let pb = points[b];
                let distance = pb.0.abs_diff(pa.0).pow(2)
                    + pb.1.abs_diff(pa.1).pow(2)
                    + pb.2.abs_diff(pa.2).pow(2);
                ((a, b), distance)
            })
        })
        .collect::<Vec<((usize, usize), usize)>>();
    ans.sort_unstable_by_key(|(_, d)| *d);
    ans.into_iter().map(|(p, _)| p).collect()
}
