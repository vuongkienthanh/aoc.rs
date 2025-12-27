use crate::parsing::{Particle, parse_input};
use fxhash::FxHashSet as Set;
use std::collections::BTreeMap;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut t_map: BTreeMap<isize, Set<usize>> = BTreeMap::default();

    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            let pa = input.get(i).unwrap();
            let pb = input.get(j).unwrap();
            if let Some(t) = find_t(pa, pb) {
                println!("i={i:?} j={j:?} t={t}");
                t_map.entry(t).or_default().insert(i);
                t_map.entry(t).or_default().insert(j);
            }
        }
    }

    let mut removed: Set<usize> = Set::default();

    println!("t_map = {t_map:?}");

    for v in t_map.into_values() {
        let actual_v: Vec<usize> = v.difference(&removed).cloned().collect();
        if actual_v.len() > 1 {
            removed.extend(actual_v);
        }
    }
    input.len() - removed.len()
}

fn find_t(a: &Particle, b: &Particle) -> Option<isize> {
    let mut t = [None; 3];

    for axis in [0, 1, 2] {
        // binomial equation
        let coeff_2a = a.a[axis] - b.a[axis];
        let coeff_b = a.v[axis] - b.v[axis];
        let coeff_c = a.p[axis] - b.p[axis];

        if coeff_2a == 0 {
            if coeff_b == 0 {
                if coeff_c != 0 {
                    return None;
                } else {
                    continue;
                }
            } else {
                if -coeff_c % coeff_b == 0 {
                    t[axis] = Some(-coeff_c / coeff_b);
                } else {
                    return None;
                }
            }
        } else {
            let delta = coeff_b.pow(2u32) - 2 * coeff_2a * coeff_c;
            if delta < 0 {
                return None;
            }
            let delta_sqrt = delta.isqrt();
            if delta_sqrt.pow(2u32) != delta {
                return None;
            }
            if (-coeff_b + delta_sqrt) % coeff_2a == 0 {
                t[axis] = Some((-coeff_b + delta_sqrt) / coeff_2a);
            } else {
                return None;
            }
        }
        println!("axis = {axis} t = {t:?}");
    }
    let t: Vec<_> = t.into_iter().flatten().collect();

    if t.iter().any(|x| *x < 0) {
        return None;
    }
    if t.len() == 1 {
        Some(t[0])
    } else if t.windows(2).all(|v| v[0] == v[1]) {
        Some(t[0])
    } else {
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 1);
    }
}
