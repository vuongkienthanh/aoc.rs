use std::collections::BTreeSet;

pub fn process(_input: &str) -> usize {
    // after studying the assembly
    // the most important register is 3 and 5 (d ,f)
    //
    // each time d div 256, f changes (small_renew)
    // when d reach less then 256
    // check eqrr 5 and 0, if eq it halts
    // else d and f change (big_renew)
    //
    // seen_f is to determine the last seen unique f
    // seen_d_f is to detect loop

    let mut seen_f = BTreeSet::new();
    let mut seen_d_f = BTreeSet::new();
    let mut ans = 0;

    let (mut d, mut f) = big_renew(0);

    loop {
        d /= 256;
        f = small_renew(d, f);
        if 256 > d {
            if seen_f.insert(f) {
                ans = f;
            }
            if !seen_d_f.insert((d, f)) {
                break;
            }
            (d, f) = big_renew(f);
        }
    }

    ans
}

fn big_renew(mut f: usize) -> (usize, usize) {
    let d = f | 65536;
    f = 733884;
    let b = d & 255;
    f = (((f + b) & 16777215) * 65899) & 16777215;
    (d, f)
}
fn small_renew(d: usize, mut f: usize) -> usize {
    let b = d & 255;
    f = (((f + b) & 16777215) * 65899) & 16777215;
    f
}
