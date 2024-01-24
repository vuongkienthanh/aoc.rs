type Numtype = f64;
type Coord = (Numtype, Numtype, Numtype);
type Coord2D = (Numtype, Numtype);

#[derive(Debug, Clone)]
struct Hailstone {
    pos: Coord,
    velo: Coord,
}
#[derive(Debug, Clone)]
struct Hailstone2D {
    pos: Coord2D,
    velo: Coord2D,
}

impl Hailstone {
    fn to_2D(&self) -> Hailstone2D {
        Hailstone2D {
            pos: (self.pos.0, self.pos.1),
            velo: (self.velo.0, self.velo.1),
        }
    }
    fn is_parallel(&self, other: &Hailstone) -> bool {
        self.velo.0 / other.velo.0 == self.velo.1 / other.velo.1
            && self.velo.0 / other.velo.0 == self.velo.2 / other.velo.2
    }
}

impl Hailstone2D {
    fn minus_velo(mut self, vrx: isize, vry: isize) -> Self {
        self.velo = (self.velo.0 - vrx as f64, self.velo.1 - vry as f64);
        self
    }
    fn intersect(&self, other: &Self) -> (Numtype, Numtype, Coord2D) {
        let t1 = (other.velo.1 * (other.pos.0 - self.pos.0)
            - other.velo.0 * (other.pos.1 - self.pos.1))
            / (self.velo.0 * other.velo.1 - other.velo.0 * self.velo.1);
        let t2 = (self.pos.1 - other.pos.1 + t1 * self.velo.1) / other.velo.1;
        let intersection = (self.pos.0 + t1 * self.velo.0, self.pos.1 + t1 * self.velo.1);
        (t1, t2, intersection)
    }
}
fn parse_input(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" @ ").unwrap();
            let mut leftiter = left.split(", ");
            let pos = (
                leftiter.next().unwrap().trim().parse::<Numtype>().unwrap(),
                leftiter.next().unwrap().trim().parse::<Numtype>().unwrap(),
                leftiter.next().unwrap().trim().parse::<Numtype>().unwrap(),
            );
            let mut rightiter = right.split(", ");
            let velo = (
                rightiter.next().unwrap().trim().parse::<Numtype>().unwrap(),
                rightiter.next().unwrap().trim().parse::<Numtype>().unwrap(),
                rightiter.next().unwrap().trim().parse::<Numtype>().unwrap(),
            );
            Hailstone { pos, velo }
        })
        .collect()
}
pub fn process(_input: &str) -> usize {
    let parsed = parse_input(_input);
    let mut parsed_iter = parsed.into_iter();
    let hailstone_a = parsed_iter.next().unwrap();
    let hailstone_b = parsed_iter.next().unwrap();
    let hailstone_c = parsed_iter.next().unwrap();

    assert!(!hailstone_a.is_parallel(&hailstone_b));
    assert!(!hailstone_a.is_parallel(&hailstone_c));

    // simulate vrx and vry
    for vrx in -1000..=1000 {
        for vry in -1000..=1000 {
            let new_hailstone2D_a = hailstone_a.to_2D().minus_velo(vrx, vry);
            let new_hailstone2D_b = hailstone_b.to_2D().minus_velo(vrx, vry);
            let new_hailstone2D_c = hailstone_c.to_2D().minus_velo(vrx, vry);

            let (timea1, timeb, intersection_ab) = new_hailstone2D_a.intersect(&new_hailstone2D_b);
            let (timea2, timec, intersection_ac) = new_hailstone2D_a.intersect(&new_hailstone2D_c);

            if intersection_ac == intersection_ab
                && timea1 >= 0.0
                && timea1 == timea2
                && timeb >= 0.0
                && timec >= 0.0
            {
                for vrz in -1000..=1000 {
                    let new_hailstone_a_z_axis = hailstone_a.pos.2 + timea1 * (hailstone_a.velo.2 - vrz as f64);
                    let new_hailstone_b_z_axis = hailstone_b.pos.2 + timeb * (hailstone_b.velo.2 - vrz as f64);
                    let new_hailstone_c_z_axis = hailstone_c.pos.2 + timec * (hailstone_c.velo.2 - vrz as f64);

                    if new_hailstone_a_z_axis == new_hailstone_b_z_axis 
                        && new_hailstone_b_z_axis == new_hailstone_c_z_axis {
                            return (intersection_ab.0 + intersection_ac.1 + new_hailstone_a_z_axis) as usize
                    }
                }
            }
        }
    }

    0
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;
        assert_eq!(process(input), 47);
    }
}
