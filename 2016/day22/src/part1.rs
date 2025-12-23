use crate::parsing::{Item, parse_input};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut ans = 0;
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            if is_viable(input[i], input[j]) {
                ans += 1;
            }
        }
    }
    ans
}

// compare used and avail
fn transferable(a: Item, b: Item) -> bool {
    a.2 > 0 && a.2 <= b.3
}
//
fn is_viable(a: Item, b: Item) -> bool {
    transferable(a, b) || transferable(b, a)
}
