use crate::{Beam, Coord, Direction};
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter;

fn shine(
    src: &Beam,
    map: &str,
    cache: &mut HashMap<Beam, (HashSet<Beam>, HashSet<Coord>)>,
) -> (HashSet<Beam>, HashSet<Coord>) {
    if let Some(res) = cache.get(src) {
        return res.clone();
    } else {
        let max_rows = map.lines().count();
        let max_cols = map.lines().next().unwrap().len();
        let mut beams = VecDeque::new();
        let mut dst = HashSet::new();
        let mut tiles = HashSet::new();

        let point = map
            .lines()
            .nth(src.loc.0)
            .unwrap()
            .chars()
            .nth(src.loc.1)
            .unwrap();
        for beam in src.encounter(point, max_rows, max_cols) {
            beams.push_front(beam);
        }

        while let Some(beam) = beams.pop_front() {
            tiles.insert(beam.loc);
            let point = map
                .lines()
                .nth(beam.loc.0)
                .unwrap()
                .chars()
                .nth(beam.loc.1)
                .unwrap();
            if point == '.' {
                if let Some(nxt_beam) = beam.next_beam(max_rows, max_cols) {
                    beams.push_back(nxt_beam);
                }
            } else {
                dst.insert(beam);
            }
        }
        cache.insert(src.clone(), (dst.clone(), tiles.clone()));
        return (dst, tiles);
    }
}

pub fn process(_input: &str) -> usize {
    let max_rows = _input.lines().count();
    let max_cols = _input.lines().next().unwrap().len();
    let mut cache: HashMap<Beam, (Vec<Beam>, HashSet<Coord>)> = HashMap::new();

    todo!();
    // let row_indices = iter::repeat(0)
    //     .take(max_cols)
    //     .chain(0..max_rows)
    //     .chain(iter::repeat(max_rows - 1).take(max_cols))
    //     .chain((0..max_rows).rev());
    // let col_indices = (0..max_cols)
    //     .chain(iter::repeat(max_cols - 1).take(max_rows))
    //     .chain((0..max_cols).rev())
    //     .chain(iter::repeat(0).take(max_rows));
    // let direction_to_test = iter::repeat(Direction::Down)
    //     .take(max_cols)
    //     .chain(iter::repeat(Direction::Left).take(max_rows))
    //     .chain(iter::repeat(Direction::Up).take(max_cols))
    //     .chain(iter::repeat(Direction::Right).take(max_rows));
    // // iterator ((row,col), direction)
    // row_indices
    //     .zip(col_indices)
    //     .zip(direction_to_test)
    //     .map(|((row, col), dir)| {
    //         let mut energized_tiles: HashSet<Coord> = HashSet::new();
    //         let mut encountered: HashSet<Beam> = HashSet::new();
    //
    //         let mut beams = VecDeque::from([Beam {
    //             loc: (row, col),
    //             direction: dir,
    //         }]);
    //
    //         while let Some(beam) = beams.pop_front() {
    //             energized_tiles.insert(beam.loc);
    //             let point = _input
    //                 .lines()
    //                 .nth(beam.loc.0)
    //                 .unwrap()
    //                 .chars()
    //                 .nth(beam.loc.1)
    //                 .unwrap();
    //             if point == '.' {
    //                 if let Some(nxt_beam) = beam.next_beam(max_rows, max_cols) {
    //                     beams.push_back(nxt_beam);
    //                 }
    //             } else {
    //             }
    //         }
    //
    //         todo!("part2");
    //         energized_tiles.len()
    //     })
    //     .max()
    //     .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!(process(input), 51);
    }

    #[test]
    #[rustfmt::skip]
    fn test_shine() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        let beam = Beam { loc: (1, 2), direction: Direction::Up, };
        let mut hm = HashMap::new();
        let ret = (
            HashSet::from([
                Beam { loc: (1, 0), direction: Direction::Left, },
                Beam { loc: (1, 4), direction: Direction::Right, },
            ]),
            HashSet::from([(1, 0), (1, 1), (1, 3), (1, 4)]),
        );
        assert_eq!(shine(&beam, input, &mut hm), ret);
        assert_eq!(hm.get(&beam).unwrap(), &ret)
    }
    #[test]
    #[rustfmt::skip]
    fn test_shine_2() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        let beam = Beam { loc: (6, 6), direction: Direction::Right, };
        let mut hm = HashMap::new();
        let ret = (
            HashSet::from([Beam { loc: (8, 6), direction: Direction::Down, }]),
            HashSet::from([(7, 6), (8, 6)]),
        );
        assert_eq!(shine(&beam, input, &mut hm), ret);
        assert_eq!(hm.get(&beam).unwrap(), &ret);
    }
}
