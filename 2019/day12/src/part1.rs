use crate::Moon;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    moonstep(_input, 1000)
}

fn moonstep(input: &str, step: usize) -> usize {
    let input = parse_input(input);
    let mut moons: Vec<Moon> = input.into_iter().map(|v| Moon::new(v)).collect();

    for _ in 0..step {
        apply_gravity(&mut moons);
        apply_velocity(&mut moons);
    }
    energy(&moons)
}

fn apply_gravity(moons: &mut [Moon]) {
    for i in 0..moons.len() - 1 {
        for j in i + 1..moons.len() {
            let (a, b) = (moons.get(i).unwrap(), moons.get(j).unwrap());
            if a.0.pos < b.0.pos {
                moons.get_mut(i).unwrap().0.vel += 1;
                moons.get_mut(j).unwrap().0.vel -= 1;
            } else if a.0.pos > b.0.pos {
                moons.get_mut(i).unwrap().0.vel -= 1;
                moons.get_mut(j).unwrap().0.vel += 1;
            }

            let (a, b) = (moons.get(i).unwrap(), moons.get(j).unwrap());
            if a.1.pos < b.1.pos {
                moons.get_mut(i).unwrap().1.vel += 1;
                moons.get_mut(j).unwrap().1.vel -= 1;
            } else if a.1.pos > b.1.pos {
                moons.get_mut(i).unwrap().1.vel -= 1;
                moons.get_mut(j).unwrap().1.vel += 1;
            }

            let (a, b) = (moons.get(i).unwrap(), moons.get(j).unwrap());
            if a.2.pos < b.2.pos {
                moons.get_mut(i).unwrap().2.vel += 1;
                moons.get_mut(j).unwrap().2.vel -= 1;
            } else if a.2.pos > b.2.pos {
                moons.get_mut(i).unwrap().2.vel -= 1;
                moons.get_mut(j).unwrap().2.vel += 1;
            }
        }
    }
}
fn apply_velocity(moons: &mut [Moon]) {
    for moon in moons.iter_mut() {
        moon.0.pos += moon.0.vel;
        moon.1.pos += moon.1.vel;
        moon.2.pos += moon.2.vel;
    }
}

fn energy(moons: &[Moon]) -> usize {
    moons
        .iter()
        .map(|moon| {
            let potential =
                moon.0.pos.unsigned_abs() + moon.1.pos.unsigned_abs() + moon.2.pos.unsigned_abs();
            let kinetic =
                moon.0.vel.unsigned_abs() + moon.1.vel.unsigned_abs() + moon.2.vel.unsigned_abs();
            potential * kinetic
        })
        .sum()
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
        10,
        179
    )]
    #[case(
        r#"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"#,
        100,
        1940
    )]
    fn test_moonstep(#[case] input: &str, #[case] step: usize, #[case] expected: usize) {
        assert_eq!(moonstep(input, step), expected);
    }
}
