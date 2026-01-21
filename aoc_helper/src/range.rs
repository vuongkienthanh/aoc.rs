//! This module provides function to works with ranges represented as a tuple of (usize, usize)

use num_traits::{One, PrimInt};

/// merge 2 ranges together, if success return the merged range, otherwise return None
pub fn merge_two_ranges<T>(a: (T, T), b: (T, T)) -> Option<(T, T)>
where
    T: PrimInt + One,
{
    if a.0 <= b.0 && a.1 >= b.1 {
        return Some((a.0, a.1));
    }
    if a.0 >= b.0 && a.1 <= b.1 {
        return Some((b.0, b.1));
    }
    if a.0 <= b.0 && a.1 + One::one() >= b.0 && a.1 <= b.1 {
        return Some((a.0, b.1));
    }
    if a.0 >= b.0 && a.0 <= b.1 + One::one() && a.1 >= b.1 {
        return Some((b.0, a.1));
    }
    None
}

/// merge all ranges in `input`
pub fn merge<T>(mut input: Vec<(T, T)>) -> Vec<(T, T)>
where
    T: PrimInt + One,
{
    let mut ans = vec![];
    input.sort_unstable_by(|(a, _), (b, _)| b.cmp(a));
    let mut current = input.pop().unwrap();
    while let Some(r) = input.pop() {
        if let Some(merged) = merge_two_ranges(current, r) {
            current = merged;
        } else {
            ans.push(current);
            current = r;
        }
    }
    ans.push(current);
    ans
}
