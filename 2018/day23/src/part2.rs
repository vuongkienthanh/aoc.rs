#![allow(non_snake_case, clippy::assign_op_pattern)]
use crate::manhattan;
use crate::parsing::{Item, parse_input};
use std::cmp::Ordering;

pub fn process(_input: &str) -> usize {
    let mut input = parse_input(_input);
    let cluster = loop {
        let mut to_removed: Vec<usize> = vec![];
        let mut min_count = usize::MAX;
        let mut cs = vec![];
        for (i, (p, r)) in input.iter().enumerate() {
            let c = input
                .iter()
                .filter(|(p2, r2)| manhattan(*p, *p2) <= r + r2)
                .count();
            cs.push(c);
            match c.cmp(&min_count) {
                Ordering::Less => {
                    min_count = c;
                    to_removed = vec![i];
                }
                Ordering::Equal => {
                    to_removed.push(i);
                }
                Ordering::Greater => (),
            }
        }
        if cs.into_iter().all(|c| c == input.len()) {
            break input;
        }
        input = input
            .into_iter()
            .enumerate()
            .filter_map(|(i, p)| (!to_removed.contains(&i)).then_some(p))
            .collect();
    };
    println!("cluster len = {:?}", cluster.len());

    let max_ilog = cluster
        .iter()
        .map(|((a, b, c), r)| {
            [a.unsigned_abs(), b.unsigned_abs(), c.unsigned_abs(), *r]
                .into_iter()
                .map(|x| x.ilog10())
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    println!("max_ilog = {max_ilog:?}\n");

    let (mut min_X, mut max_X, mut min_Y, mut max_Y, mut min_Z, mut max_Z) = {
        let d = max_ilog.saturating_sub(3);
        println!("precompute at d= {d}");
        let div = 10i32.pow(d) as isize;
        let new_cluster: Vec<Item> = cluster
            .iter()
            .map(|((a, b, c), r)| ((a / div, b / div, c / div), r / div as usize))
            .collect();
        let (min_X, max_X, min_Y, max_Y, min_Z, max_Z) = find_min_max_in_cluster(&new_cluster);
        println!("bgn stats = {min_X} {max_X} {min_Y} {max_Y} {min_Z} {max_Z}");
        let (min_X, max_X, min_Y, max_Y, min_Z, max_Z) = find_viable_min_max_in_cluster(
            &new_cluster,
            d as usize,
            (min_X, max_X, min_Y, max_Y, min_Z, max_Z),
        );
        println!("end stats = {min_X} {max_X} {min_Y} {max_Y} {min_Z} {max_Z}\n");
        (min_X, max_X, min_Y, max_Y, min_Z, max_Z)
    };

    for d in (0..max_ilog.saturating_sub(3)).rev() {
        println!("at d= {d}");
        min_X = min_X * 10;
        max_X = max_X * 10 + 9;
        min_Y = min_Y * 10;
        max_Y = max_Y * 10 + 9;
        min_Z = min_Z * 10;
        max_Z = max_Z * 10 + 9;
        println!("bgn stats = {min_X} {max_X} {min_Y} {max_Y} {min_Z} {max_Z}");

        let div = 10i32.pow(d) as isize;
        let new_cluster: Vec<Item> = cluster
            .iter()
            .map(|((a, b, c), r)| ((a / div, b / div, c / div), r / div as usize))
            .collect();

        (min_X, max_X, min_Y, max_Y, min_Z, max_Z) = find_viable_min_max_in_cluster(
            &new_cluster,
            d as usize,
            (min_X, max_X, min_Y, max_Y, min_Z, max_Z),
        );
        println!("end stats = {min_X} {max_X} {min_Y} {max_Y} {min_Z} {max_Z}");
        println!();
    }
    let mut ans = usize::MAX;

    'i: for x in min_X..=max_X {
        for y in min_Y..=max_Y {
            for z in min_Z..=max_Z {
                if cluster
                    .iter()
                    .all(|((a, b, c), r)| x.abs_diff(*a) + y.abs_diff(*b) + z.abs_diff(*c) <= *r)
                {
                    ans = ans.min((x + y + z) as usize);
                }
            }
        }
    }
    ans
}

fn find_min_max_in_cluster(cluster: &[Item]) -> (isize, isize, isize, isize, isize, isize) {
    cluster.iter().fold(
        (
            isize::MIN,
            isize::MAX,
            isize::MIN,
            isize::MAX,
            isize::MIN,
            isize::MAX,
        ),
        |(min_x, max_x, min_y, max_y, min_z, max_z), ((a, b, c), r)| {
            (
                min_x.max(a.checked_sub_unsigned(*r).unwrap()),
                max_x.min(a.checked_add_unsigned(*r).unwrap()),
                min_y.max(b.checked_sub_unsigned(*r).unwrap()),
                max_y.min(b.checked_add_unsigned(*r).unwrap()),
                min_z.max(c.checked_sub_unsigned(*r).unwrap()),
                max_z.min(c.checked_add_unsigned(*r).unwrap()),
            )
        },
    )
}

fn find_viable_min_max_in_cluster(
    cluster: &[Item],
    noise: usize,
    (min_X, max_X, min_Y, max_Y, min_Z, max_Z): (isize, isize, isize, isize, isize, isize),
) -> (isize, isize, isize, isize, isize, isize) {
    let (mut min_x, mut max_x, mut min_y, mut max_y, mut min_z, mut max_z) = (
        isize::MAX,
        isize::MIN,
        isize::MAX,
        isize::MIN,
        isize::MAX,
        isize::MIN,
    );

    for x in min_X..=max_X {
        for y in min_Y..=max_Y {
            for z in min_Z..=max_Z {
                // in range of all nodes, with noise
                if cluster.iter().all(|((a, b, c), r)| {
                    x.abs_diff(*a) + y.abs_diff(*b) + z.abs_diff(*c) <= *r + noise
                }) {
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                    min_y = min_y.min(y);
                    max_y = max_y.max(y);
                    min_z = min_z.min(z);
                    max_z = max_z.max(z);
                }
            }
        }
    }
    (min_x, max_x, min_y, max_y, min_z, max_z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 36);
    }
}
