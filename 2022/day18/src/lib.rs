pub mod parsing;
pub mod part1;
pub mod part2;
use parsing::Item;
use std::collections::BTreeSet;

/// input sorted by (z, y, x)
pub fn naive_surface(input: &BTreeSet<Item>) -> usize {
    let mut ans = 0;
    let mut bottom = vec![];
    let mut top = vec![];
    let mut cz = input.first().unwrap().0;

    // assume each z layer is diff by 1;
    for (z, y, x) in input {
        if *z != cz {
            bottom = std::mem::take(&mut top);
            cz = *z;
        }
        ans += 6;
        if bottom.contains(&(*x, *y)) {
            ans -= 2
        }
        if top.contains(&(x - 1, *y)) {
            ans -= 2
        }
        if top.contains(&(*x, y - 1)) {
            ans -= 2
        }
        top.push((*x, *y));
    }

    ans
}
