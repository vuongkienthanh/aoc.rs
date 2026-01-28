use crate::parsing::{Cell, parse};
use aoc_helper::adj::grid::adj4;

pub fn process(_input: &str) -> usize {
    let (map, portals, start, end) = parse(_input);
    let rows = map.len();
    let cols = map[0].len();

    let mut seen = vec![vec![false; cols]; rows];

    let mut current = vec![(start, 0, false)];

    'a: loop {
        let mut new = vec![];

        for ((r, c), step, teleported) in current {
            if (r, c) == end {
                break 'a step;
            }
            match map[r][c] {
                Cell::Wall => (),
                Cell::Space => {
                    if !seen[r][c] {
                        for p2 in adj4((r, c), rows, cols).into_iter().flatten() {
                            new.push((p2, step + 1, teleported));
                        }
                        seen[r][c] = true;
                    }
                }
                Cell::InnerPortal(pt) | Cell::OuterPortal(pt) => {
                    if !seen[r][c] {
                        if teleported {
                            for p2 in adj4((r, c), rows, cols).into_iter().flatten() {
                                new.push((p2, step + 1, false));
                            }
                        } else {
                            let p2 = portals
                                .get(&pt)
                                .unwrap()
                                .iter()
                                .find(|p| p != &&(r, c))
                                .unwrap();
                            new.push((*p2, step + 1, true));
                        }
                        seen[r][c] = true;
                    }
                }
            }
        }

        current = new;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       "#,
        23
    )]
    #[case(
        r#"                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               "#,
        58
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
