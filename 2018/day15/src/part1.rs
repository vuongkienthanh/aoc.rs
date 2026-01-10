use crate::parsing::{Item, parse_input};
use crate::{display_map, get_units, run_once};

pub fn process(_input: &str) -> usize {
    let mut map = parse_input(_input);
    display_map(&map);
    for i in 0.. {
        if run_once(&mut map) {
            let hp = get_units(&map)
                .into_iter()
                .map(|(r, c)| map[r][c].hp())
                .sum::<usize>();
            println!("{hp} {i}");
            display_map(&map);
            // println!("{map:?}");
            return hp * i;
        }
        println!("{i}");
        display_map(&map);
    }
    panic!("should have an answer")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"#,
        36334
    )]
    #[case(
        r#"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"#,
        39514
    )]
    #[case(
        r#"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"#,
        27755
    )]
    #[case(
        r#"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"#,
        28944
    )]
    #[case(
        r#"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"#,
        18740
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        let mut map = parse_input(input);
        let mut ans = None;
        for i in 0.. {
            if run_once(&mut map) {
                let hp = get_units(&map)
                    .into_iter()
                    .map(|(r, c)| map[r][c].hp())
                    .sum::<usize>();
                ans = Some(hp * i);
                break;
            }
        }
        assert_eq!(ans.unwrap(), expected);
    }
}
