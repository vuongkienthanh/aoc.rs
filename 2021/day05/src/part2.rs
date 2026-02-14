use crate::parsing::parse_input;
use fxhash::FxHashMap;
use std::iter::Iterator;
type Map = FxHashMap<(usize, usize), usize>;
type XYrange = (
    Box<dyn Iterator<Item = usize>>,
    Box<dyn Iterator<Item = usize>>,
);

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input
        .into_iter()
        .fold(Map::default(), |mut acc, ((x1, y1), (x2, y2))| {
            if x1 == x2 {
                let range = if y1 <= y2 { y1..=y2 } else { y2..=y1 };
                for y in range {
                    *acc.entry((x1, y)).or_default() += 1;
                }
                acc
            } else if y1 == y2 {
                let range = if x1 <= x2 { x1..=x2 } else { x2..=x1 };
                for x in range {
                    *acc.entry((x, y1)).or_default() += 1;
                }
                acc
            } else {
                let (xrange, yrange): XYrange = match ((x1 <= x2), (y1 <= y2)) {
                    (true, true) => (Box::new(x1..=x2), Box::new(y1..=y2)),
                    (true, false) => (Box::new(x1..=x2), Box::new((y2..=y1).rev())),
                    (false, true) => (Box::new((x2..=x1).rev()), Box::new(y1..=y2)),
                    (false, false) => (Box::new((x2..=x1).rev()), Box::new((y2..=y1).rev())),
                };
                for (x, y) in xrange.zip(yrange) {
                    *acc.entry((x, y)).or_default() += 1;
                }
                acc
            }
        })
        .into_values()
        .filter(|x| *x >= 2)
        .count()
}
