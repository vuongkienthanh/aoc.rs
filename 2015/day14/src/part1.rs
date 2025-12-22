use crate::parsing::parse_input;

const TIME: usize = 2503;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    input
        .into_iter()
        .map(|(v, t1, t2)| distance_traveled(TIME, v, t1, t2))
        .max()
        .unwrap()
}

fn distance_traveled(time: usize, v: usize, t1: usize, t2: usize) -> usize {
    let big_t = t1 + t2;
    let big_t_count = time / big_t;
    let remain = time % big_t;
    (big_t_count*t1 + remain.min(t1)) * v
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(14, 10, 127, 1120)]
    #[case(16, 11, 162, 1056)]
    fn test_process(
        #[case] v: usize,
        #[case] t1: usize,
        #[case] t2: usize,
        #[case] expected: usize,
    ) {
        assert_eq!(distance_traveled(1000, v,t1,t2), expected);
    }
}
