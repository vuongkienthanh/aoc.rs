use crate::Axis;
use crate::parsing::parse_input;
use aoc_helper::algebra::lcm_vec;
use fxhash::FxHashSet;

pub fn process(_input: &str) -> u64 {
    let input = parse_input(_input);
    let (xs, ys, zs) = input.into_iter().fold(
        (vec![], vec![], vec![]),
        |(mut xs, mut ys, mut zs), (x, y, z)| {
            xs.push(Axis { pos: x, vel: 0 });
            ys.push(Axis { pos: y, vel: 0 });
            zs.push(Axis { pos: z, vel: 0 });
            (xs, ys, zs)
        },
    );
    let found_x = find_loop(xs);
    let found_y = find_loop(ys);
    let found_z = find_loop(zs);
    lcm_vec(&[found_x, found_y, found_z])
}

fn find_loop(mut v: Vec<Axis>) -> u64 {
    let mut seen = FxHashSet::default();
    seen.insert(v.clone());

    let mut step = 0;
    'a: loop {
        step += 1;
        // apply_gravity
        for i in 0..v.len() - 1 {
            for j in i + 1..v.len() {
                let (a, b) = (v.get(i).unwrap(), v.get(j).unwrap());
                if a.pos < b.pos {
                    v.get_mut(i).unwrap().vel += 1;
                    v.get_mut(j).unwrap().vel -= 1;
                } else if a.pos > b.pos {
                    v.get_mut(i).unwrap().vel -= 1;
                    v.get_mut(j).unwrap().vel += 1;
                }
            }
        }
        // apply_velocity
        for a in v.iter_mut() {
            a.pos += a.vel;
        }
        if !seen.insert(v.clone()) {
            break 'a step;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#,
        2772
    )]
    #[case(
        r#"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"#,
        4686774924
    )]
    fn test_process(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(process(input), expected);
    }
}
