pub mod parsing;
pub mod part1;
use parsing::Item;

pub fn manhattan(a: Item, b: Item) -> usize {
    [
        a.0.abs_diff(b.0),
        a.1.abs_diff(b.1),
        a.2.abs_diff(b.2),
        a.3.abs_diff(b.3),
    ]
    .into_iter()
    .sum()
}
