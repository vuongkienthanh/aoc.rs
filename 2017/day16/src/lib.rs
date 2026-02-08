pub mod parsing;
pub mod part1;
pub mod part2;
use parsing::Item;

fn run(dancers: &mut [char], input: &[Item]) {
    for item in input {
        match item {
            Item::Spin(x) => {
                dancers.rotate_right(*x);
            }
            Item::Exchange(a, b) => {
                dancers.swap(*a, *b);
            }
            Item::Partner(a, b) => {
                let mut a_pos = None;
                let mut b_pos = None;
                for (i, d) in dancers.iter().enumerate() {
                    if d == a {
                        a_pos = Some(i);
                        if b_pos.is_some() {
                            break;
                        }
                    }
                    if d == b {
                        b_pos = Some(i);
                        if a_pos.is_some() {
                            break;
                        }
                    }
                }
                dancers.swap(a_pos.unwrap(), b_pos.unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::parse_input;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"s1,x3/4,pe/b"#
    }

    #[rstest]
    fn test_run(fixture: &str) {
        let mut dancers: Vec<char> = ('a'..='e').collect();
        let input = parse_input(fixture);
        run(&mut dancers, &input);
        assert!(
            dancers
                .into_iter()
                .zip("baedc".chars())
                .all(|(a, b)| a == b)
        );
    }
}
