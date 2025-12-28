use crate::parsing::{Particle, parse_input};
use fxhash::FxHashSet as Set;
use std::cmp::Ordering;
use std::collections::BTreeMap;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut t_map: BTreeMap<isize, Set<usize>> = BTreeMap::default();

    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            let pa = input.get(i).unwrap();
            let pb = input.get(j).unwrap();
            if let Some(t) = find_t(pa, pb) {
                t_map.entry(t).or_default().extend([i, j]);
            }
        }
    }

    let mut removed: Set<usize> = Set::default();

    for v in t_map.into_values() {
        let actual_v: Vec<usize> = v.difference(&removed).cloned().collect();
        if actual_v.len() > 1 {
            removed.extend(actual_v);
        }
    }
    input.len() - removed.len()
}

fn find_t(a: &Particle, b: &Particle) -> Option<isize> {
    let mut t: Vec<Set<isize>> = vec![];

    // binomial equation for discrete
    for axis in [0, 1, 2] {
        let coeff_a = a.a[axis] - b.a[axis];
        let coeff_b = 2 * (a.v[axis] - b.v[axis]) + coeff_a;
        let coeff_c = 2 * (a.p[axis] - b.p[axis]);
        let mut ts = Set::default();

        match (coeff_a, coeff_b, coeff_c) {
            (0, 0, 0) => {
                continue;
            }
            (0, 0, _) => {
                return None;
            }
            (0, b, c) => {
                if -c % b == 0 {
                    ts.insert(-c / b);
                }
            }
            (a, b, c) => {
                let delta = b.pow(2u32) - 4 * a * c;
                match delta.cmp(&0) {
                    Ordering::Less => {
                        return None;
                    }
                    Ordering::Equal => {
                        if -b % (2 * a) == 0 {
                            ts.insert(-b / (2 * a));
                        }
                    }
                    Ordering::Greater => {
                        let delta_sqrt = delta.isqrt();
                        if delta_sqrt.pow(2u32) != delta {
                            // float
                            return None;
                        }
                        if (-b + delta_sqrt) % (2 * a) == 0 {
                            ts.insert((-b + delta_sqrt) / (2 * a));
                        }
                        if (-b - delta_sqrt) % (2 * a) == 0 {
                            ts.insert((-b - delta_sqrt) / (2 * a));
                        }
                    }
                }
            }
        }
        ts.retain(|x| *x >= 0);
        if ts.is_empty() {
            return None;
        }
        t.push(ts);
    }

    t.into_iter()
        .reduce(|acc, e| acc.intersection(&e).cloned().collect::<Set<isize>>())
        .unwrap()
        .into_iter()
        .next()
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
