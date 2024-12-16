use crate::parse;
use crate::CoordKey;

pub fn process(_input: &str) -> usize {
    let (grid, start, end, graph) = parse(_input);

    // for (i, line) in grid.iter_rows().enumerate() {
    //     for (j, c) in line.enumerate() {
    //         if graph.contains_key(&CoordKey::from((i, j))) {
    //             print!("X");
    //         } else {
    //             print!("{c}");
    //         }
    //     }
    //     println!();
    // }

//     for (k, v) in graph {
//         println!(
//             r#"
// ({}, {}):
//     up:{:?}
//     down:{:?}
//     left:{:?}
//     right:{:?}"#,
//             k.0, k.1, v.up, v.down, v.left, v.right
//         );
//     }

    todo!("part1")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        assert_eq!(process(input), 7036);
    }
}
