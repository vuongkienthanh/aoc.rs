use crate::parsing::{Cell, parse};
use aoc_helper::adj::grid::adj4;
use std::collections::BTreeMap;

pub fn process(_input: &str) -> usize {
    let (map, portals, start, end) = parse(_input);
    let rows = map.len();
    let cols = map[0].len();
    let mut seen: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; cols]; rows];

    let mut current = vec![(start, 0, 0, false, BTreeMap::new())];
    'a: loop {
        let mut new = vec![];

        for ((r, c), layer, step, teleported, mut walked) in current {
            if (r, c) == end && layer == 0 {
                break 'a step;
            }
            if layer > 0 && ((r, c) == start || (r, c) == end) {
                continue;
            }
            match map[r][c] {
                Cell::Wall => panic!("should not be wall"),
                Cell::Space => {
                    for p2 in adj4((r, c), rows, cols)
                        .into_iter()
                        .flatten()
                        .filter(|(r, c)| {
                            !matches!(map[*r][*c], Cell::Wall) && !seen[*r][*c].contains(&layer)
                        })
                    {
                        new.push((p2, layer, step + 1, teleported, walked.clone()));
                    }
                    seen[r][c].push(layer);
                }
                Cell::InnerPortal(_) | Cell::OuterPortal(_) if teleported => {
                    for p2 in adj4((r, c), rows, cols)
                        .into_iter()
                        .flatten()
                        .filter(|(r, c)| {
                            !matches!(map[*r][*c], Cell::Wall) && !seen[*r][*c].contains(&layer)
                        })
                    {
                        new.push((p2, layer, step + 1, false, walked.clone()));
                    }
                    seen[r][c].push(layer);
                }
                Cell::InnerPortal(pt) => {
                    if let Some(min_layer) = walked.get(&pt)
                        && layer >= *min_layer
                    {
                        continue;
                    }
                    walked.insert(pt, layer);
                    let (r2, c2) = portals
                        .get(&pt)
                        .unwrap()
                        .iter()
                        .find(|p| p != &&(r, c))
                        .cloned()
                        .unwrap();
                    if seen[r2][c2].contains(&(layer + 1)) {
                        continue;
                    }
                    new.push(((r2, c2), layer + 1, step + 1, true, walked));
                    seen[r][c].push(layer);
                }
                Cell::OuterPortal(pt) if layer > 0 => {
                    let (r2, c2) = portals
                        .get(&pt)
                        .unwrap()
                        .iter()
                        .find(|p| p != &&(r, c))
                        .cloned()
                        .unwrap();
                    if seen[r2][c2].contains(&(layer - 1)) {
                        continue;
                    }
                    new.push(((r2, c2), layer - 1, step + 1, true, walked));
                    seen[r][c].push(layer);
                }
                Cell::OuterPortal(_) => continue,
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
