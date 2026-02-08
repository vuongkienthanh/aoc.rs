use crate::parsing::parse_input;
use crate::{Map, Point, WhoDead, atk, display_map, get_units, mov};

pub fn process(_input: &str) -> usize {
    let mut map = parse_input(_input);
    for i in 0.. {
        if run_once(&mut map) {
            display_map(&map);
            let hp = map
                .iter()
                .flat_map(|line| line.iter())
                .map(|i| i.hp())
                .sum::<usize>();
            return hp * i;
        }
    }
    panic!("should have an answer")
}

fn run_once(map: &mut Map) -> bool {
    let mut dead_list: Vec<Point> = vec![];
    for unit in get_units(&*map) {
        if dead_list.contains(&unit) {
            continue;
        }
        if let Some(p) = mov(unit, map) {
            match atk(p, map, 3) {
                WhoDead::Elf(killed) | WhoDead::Goblin(killed) => {
                    dead_list.push(killed);
                }
                _ => (),
            }
        } else {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::Item;
    use rstest::*;

    #[test]
    fn test_mov() {
        let mut map = parse_input(
            r#"#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########"#,
        );
        for _ in 0..3 {
            let mut units = get_units(&map);
            units.sort_unstable();
            for unit in units {
                mov(unit, &mut map);
            }
        }

        let result_map = parse_input(
            r#"#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########"#,
        );
        display_map(&map);
        display_map(&result_map);
        assert_eq!(map, result_map);
    }

    #[test]
    fn test_mov2() {
        let mut map = parse_input(
            r#"#######
#.E...#
#.....#
#...G.#
#######"#,
        );
        mov((1, 2), &mut map);
        display_map(&map);
        assert!(matches!(map[1][3], Item::Elf(_)));
    }

    #[test]
    fn test_atk() {
        let mut map = parse_input(
            r#"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"#,
        );
        for _ in 0..28 {
            run_once(&mut map);
        }
        let mut result_map = parse_input(
            r#"#######
#G....#
#.G...#
#.#.#G#
#...#E#
#....G#
#######"#,
        );
        result_map[2][2] = Item::Goblin(131);
        result_map[3][5] = Item::Goblin(116);
        result_map[4][5] = Item::Elf(113);
        display_map(&map);
        display_map(&result_map);
        assert_eq!(map, result_map);
    }

    #[test]
    fn test_atk_2() {
        let mut map = parse_input(
            r#"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"#,
        );
        for _ in 0..48 {
            run_once(&mut map);
        }
        let mut result_map = parse_input(
            r#"#######
#G....#
#.G...#
#.#.#G#
#...#.#
#....G#
#######"#,
        );
        result_map[2][2] = Item::Goblin(131);
        result_map[3][5] = Item::Goblin(59);
        assert_eq!(map, result_map);
    }

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
        assert_eq!(process(input), expected);
    }
}
