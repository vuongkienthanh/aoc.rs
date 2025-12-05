use num_traits::{One, PrimInt};

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

pub fn merge<T>(input: Vec<(T, T)>) -> Vec<(T, T)>
where
    T: PrimInt + One,
{
    let mut ans = vec![];
    for i in &input {
        let mut merged = false;
        let mut new_ans = vec![];
        while let Some(j) = ans.pop() {
            if let Some(k) = merge_two_ranges(*i, j) {
                merged = true;
                new_ans.push(k);
                new_ans.extend(ans);
                break;
            } else {
                new_ans.push(j)
            }
        }
        if !merged {
            new_ans.push(*i);
        }
        ans = new_ans;
    }
    if input.len() == ans.len() {
        return input;
    } else {
        return merge(ans);
    }
}
