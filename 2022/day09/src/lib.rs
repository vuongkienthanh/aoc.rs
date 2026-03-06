pub mod parsing;
pub mod part1;
pub mod part2;
use aoc_helper::adj::naive::{adj4, adjx};

type Knot = (isize, isize);

fn is_touched((a, b): Knot, (x, y): Knot) -> bool {
    a.abs_diff(x) <= 1 && b.abs_diff(y) <= 1
}

fn follow(head: Knot, mut tail: Knot) -> Knot {
    if !is_touched(head, tail) {
        if let Some(x) = adj4(head).into_iter().find(|adj| is_touched(tail, *adj)) {
            tail = x;
        } else if let Some(x) = adjx(head).into_iter().find(|adj| is_touched(tail, *adj)) {
            tail = x;
        }
    }
    tail
}
