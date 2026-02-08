use crate::parsing::parse_input;

const TIME: usize = 2503;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let num_of_reindeers = input.len();
    let mut scores = vec![0; num_of_reindeers];
    let mut distances_traveled = vec![0; num_of_reindeers];
    race(TIME, &input, &mut scores, &mut distances_traveled);

    scores.into_iter().max().unwrap()
}

fn race(
    time: usize,
    reindeers: &[(usize, usize, usize)],
    scores: &mut [usize],
    distances_traveled: &mut [usize],
) {
    for t in 1..time + 1 {
        for (r, (v, t1, t2)) in reindeers.iter().enumerate() {
            let rem = t % (t1 + t2);
            if rem <= *t1 && rem != 0 {
                distances_traveled[r] += v;
            }
        }
        let max_d = *distances_traveled.iter().max().unwrap();
        for r in (0..reindeers.len()).filter(|r| distances_traveled[*r] == max_d) {
            scores[r] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(vec![(14, 10, 127) ,(16, 11, 162)], vec![1120,1056], vec![312,689])]
    fn test_race(
        #[case] reindeers: Vec<(usize, usize, usize)>,
        #[case] expected_distances: Vec<usize>,
        #[case] expected_scores: Vec<usize>,
    ) {
        let num_of_reindeers = reindeers.len();
        let mut distances_traveled = vec![0; num_of_reindeers];
        let mut scores = vec![0; num_of_reindeers];
        race(1000, &reindeers, &mut scores, &mut distances_traveled);
        assert_eq!(distances_traveled, expected_distances, "distance");
        assert_eq!(scores, expected_scores, "score");
    }
}
