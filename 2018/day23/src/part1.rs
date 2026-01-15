use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let (origin, max_r) = input.iter().max_by_key(|(_, r)| *r).cloned().unwrap();
    input
        .into_iter()
        .filter(|(p, _)| manhattan(*p, origin) <= max_r)
        .count()
}

fn manhattan(point: (isize, isize, isize), origin: (isize, isize, isize)) -> usize {
    point.0.abs_diff(origin.0) + point.1.abs_diff(origin.1) + point.2.abs_diff(origin.2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 7);
    }
}
