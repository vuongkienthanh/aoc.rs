type Numtype = f64;
type Coord = (Numtype, Numtype);
#[derive(Debug)]
struct Hailstone {
    pos: Coord,
    velo: Coord,
}

impl Hailstone {
    fn is_parallel(&self, other: &Hailstone) -> bool {
        self.velo.0 / other.velo.0 == self.velo.1 / other.velo.1
    }
    fn intersect(&self, other: &Hailstone) -> (Numtype, Numtype, Coord) {
        assert!(!self.is_parallel(other));

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
            );
            let mut rightiter = right.split(", ");
            let velo = (
                rightiter.next().unwrap().trim().parse::<Numtype>().unwrap(),
                rightiter.next().unwrap().trim().parse::<Numtype>().unwrap(),
            );
            Hailstone { pos, velo }
        })
        .collect()
}
pub fn process(_input: &str, xmin: Numtype, xmax: Numtype, ymin: Numtype, ymax: Numtype) -> usize {
    let parsed = parse_input(_input);
    // dbg!(parsed);

    let mut ret = 0;

    for i in 0..parsed.len() {
        for j in (i + 1)..parsed.len() {
            let one = parsed.get(i).unwrap();
            let two = parsed.get(j).unwrap();

            if one.is_parallel(two) {
                continue;
            } else {
                let (t1, t2, intersection) = one.intersect(two);
                if t1 >= 0.0
                    && t2 >= 0.0
                    && intersection.0 >= xmin
                    && intersection.0 <= xmax
                    && intersection.1 >= ymin
                    && intersection.1 <= ymax
                {
                    ret += 1
                }
            }
        }
    }

    ret
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
        assert_eq!(process(input, 7.0, 27.0, 7.0, 27.0), 2);
    }
}
