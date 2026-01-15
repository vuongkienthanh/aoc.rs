use crate::parsing::parse_input;
use crate::{Point, manhattan};
use itertools::Itertools;
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
    // for i in &cluster {
    //     println!("{i:?}");
    // }
    println!("cluster len = {:?}", cluster.len());

    // let min_pair = cluster
    //     .iter()
    //     .combinations(2)
    //     .min_by_key(|v| {
    //         let (p0, r0) = v[0];
    //         let (p1, r1) = v[1];
    //         r0 + r1 - manhattan(*p0, *p1)
    //     })
    //     .unwrap();
    // let (min_x, max_x, min_y, max_y, min_z, max_z) = {
    //     let ((x0, y0, z0), r0) = min_pair[0];
    //     let ((x1, y1, z1), r1) = min_pair[1];
    //     (
    //         (x0.checked_sub_unsigned(*r0).unwrap()).max(x1.checked_sub_unsigned(*r1).unwrap()),
    //         (x0.checked_add_unsigned(*r0).unwrap()).min(x1.checked_add_unsigned(*r1).unwrap()),
    //         (y0.checked_sub_unsigned(*r0).unwrap()).max(y1.checked_sub_unsigned(*r1).unwrap()),
    //         (y0.checked_add_unsigned(*r0).unwrap()).min(y1.checked_add_unsigned(*r1).unwrap()),
    //         (z0.checked_sub_unsigned(*r0).unwrap()).max(z1.checked_sub_unsigned(*r1).unwrap()),
    //         (z0.checked_add_unsigned(*r0).unwrap()).min(z1.checked_add_unsigned(*r1).unwrap()),
    //     )
    // };
    // println!("min_pair = {min_pair:?}");
    // println!("{:?}", (min_x, max_x, min_y, max_y, min_z, max_z));

    let (min_x, max_x, min_y, max_y, min_z, max_z) = cluster.iter().fold(
        (
            isize::MIN,
            isize::MAX,
            isize::MIN,
            isize::MAX,
            isize::MIN,
            isize::MAX,
        ),
        |(min_x, max_x, min_y, max_y, min_z, max_z), ((x, y, z), r)| {
            (
                min_x.max(x.checked_sub_unsigned(*r).unwrap()),
                max_x.min(x.checked_add_unsigned(*r).unwrap()),
                min_y.max(y.checked_sub_unsigned(*r).unwrap()),
                max_y.min(y.checked_add_unsigned(*r).unwrap()),
                min_z.max(z.checked_sub_unsigned(*r).unwrap()),
                max_z.min(z.checked_add_unsigned(*r).unwrap()),
            )
        },
    );
    println!("{:?}", (min_x, max_x, min_y, max_y, min_z, max_z));

    let (mut min_z, mut max_z) = (isize::MIN, isize::MAX);
    let mut ans = usize::MAX;
    'xi: for x in min_x..=max_x {
        'yi: for y in min_y..=max_y {
            for ((a, b, c), r) in &cluster {
                if *r < a.abs_diff(x) + b.abs_diff(y) {
                    println!("skip x y  = {x} {y}");
                    continue 'yi;
                }
                // println!("{x} {y} with cluster {a} {b} {c} {r}");
                // |z-c|
                let z_c_abs = r - a.abs_diff(x) - b.abs_diff(y);
                // println!("z_c_abs = {z_c_abs}");
                min_z = min_z.max(-(z_c_abs as isize) + c);
                max_z = max_z.min((z_c_abs as isize) + c);
            }
            println!(" z = [{min_z}, {max_z}]");

            ans = ans.min(
                x.unsigned_abs()
                    + y.unsigned_abs()
                    + min_z.unsigned_abs().min(max_z.unsigned_abs()),
            );
        }
    }
    println!("ans= {ans}");
    panic!("should have an answer")
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
