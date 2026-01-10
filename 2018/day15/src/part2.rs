use crate::parsing::parse_input;
use crate::{Map, Point, WhoDead, atk, display_map, get_units, mov, parsing::Item};

pub fn process(_input: &str) -> usize {
    let map = parse_input(_input);
    'a: for atk_power in 4.. {
        let mut map = map.clone();
        for i in 0.. {
            match run_once(&mut map, atk_power) {
                IsFinish::ElfDead => {
                    continue 'a;
                }
                IsFinish::Yes => {
                    display_map(&map);
                    let hp = map
                        .iter()
                        .flat_map(|line| line.iter())
                        .map(|i| i.hp())
                        .sum::<usize>();
                    println!("atk_power={atk_power}");
                    return hp * i;
                }
                IsFinish::No => (),
            }
        }
    }
    panic!("should have an answer")
}

enum IsFinish {
    ElfDead,
    Yes,
    No,
}

fn run_once(map: &mut Map, atk_power: usize) -> IsFinish {
    let mut dead_list: Vec<Point> = vec![];
    for unit in get_units(&*map) {
        if dead_list.contains(&unit) {
            continue;
        }
        if let Some(p) = mov(unit, map) {
            let atk_power = if matches!(map[p.0][p.1], Item::Elf(_)) {
                atk_power
            } else {
                3
            };
            match atk(p, map, atk_power) {
                WhoDead::Elf(_) => {
                    return IsFinish::ElfDead;
                }
                WhoDead::Goblin(killed) => {
                    dead_list.push(killed);
                }
                WhoDead::No => (),
            }
        } else {
            return IsFinish::Yes;
        }
    }
    IsFinish::No
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"#,
        4988
    )]
    #[case(
        r#"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"#,
        31284
    )]
    #[case(
        r#"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"#,
        3478
    )]
    #[case(
        r#"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"#,
        6474
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
        1140
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
