use crate::parsing::{Cell, parse};
use aoc_helper::adj::grid::adj4;
use fxhash::FxHashSet as Set;

pub fn process(_input: &str) -> usize {
    let (map, portals, start, end) = parse(_input);
    let rows = map.len();
    let cols = map[0].len();
    let mut seen: Vec<Vec<Set<usize>>> = vec![vec![Set::default(); cols]; rows];

    let mut current = vec![(start, 0, 0, false)];
    'a: loop {
        let mut new = vec![];

        for ((r, c), layer, step, teleported) in current {
            match map[r][c] {
                Cell::Wall => (),
                Cell::Space if layer == 0 && (r, c) == end => {
                    break 'a step;
                }
                Cell::Space => {
                    if seen[r][c].insert(layer) {
                        for p2 in adj4((r, c), rows, cols).into_iter().flatten() {
                            new.push((p2, layer, step + 1, teleported));
                        }
                    }
                }
                Cell::InnerPortal(_) | Cell::OuterPortal(_) if teleported => {
                    if seen[r][c].insert(layer) {
                        for p2 in adj4((r, c), rows, cols).into_iter().flatten() {
                            new.push((p2, layer, step + 1, false));
                        }
                    }
                }
                Cell::InnerPortal(pt) => {
                    if seen[r][c].insert(layer) {
                        let p2 = portals
                            .get(&pt)
                            .unwrap()
                            .iter()
                            .find(|p| p != &&(r, c))
                            .cloned()
                            .unwrap();
                        new.push((p2, layer + 1, step + 1, true));
                    }
                }
                Cell::OuterPortal(pt) if layer > 0 => {
                    if seen[r][c].insert(layer) {
                        let p2 = portals
                            .get(&pt)
                            .unwrap()
                            .iter()
                            .find(|p| p != &&(r, c))
                            .cloned()
                            .unwrap();
                        new.push((p2, layer - 1, step + 1, true));
                    }
                }
                Cell::OuterPortal(_) => (),
            }
            // if let Cell::InnerPortal(p) | Cell::OuterPortal(p) = map[r][c] {
            //     println!("step={step}, at {p:?} layer {layer}");
            // }
        }

        current = new;
        if current.is_empty() {
            panic!("should have an answer");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#"             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     "#,
        396
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
