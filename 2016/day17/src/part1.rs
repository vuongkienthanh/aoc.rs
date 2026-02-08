use crate::{DIRECTIONS, md5hash};
use aoc_helper::adj::grid::adj4;

pub fn process(_input: &str) -> String {
    let loc = (0, 0);
    let history = String::new();
    let mut v = vec![(loc, history)];

    while !v.is_empty() {
        let mut new_v = vec![];

        for (loc, history) in v {
            if loc == (3, 3) {
                return history;
            }

            let hex = md5hash(_input, &history);
            for (new_loc, dn, dir) in adj4(loc, 4, 4)
                .into_iter()
                .zip(DIRECTIONS)
                .filter_map(|(loc, (dn, dir))| loc.map(|p| (p, dn, dir)))
            {
                // check is_wall
                let c = hex.chars().nth(dn).unwrap();
                if !['b', 'c', 'd', 'e', 'f'].contains(&c) {
                    continue;
                }
                let mut new_history = history.clone();
                new_history.push(dir);
                new_v.push((new_loc, new_history));
            }
        }

        v = new_v;
    }

    panic!("should have an answer")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    #[rstest]
    #[case("ihgpwlah", "DDRRRD")]
    #[case("kglvqrro", "DDUDRLRRUDRD")]
    #[case("ulqzkmiv", "DRURDRUDDLLDLUURRDULRLDUUDDDRR")]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process(input).as_str(), expected);
    }
}
