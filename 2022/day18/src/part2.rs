use crate::parsing::parse_input;
use fxhash::FxHashSet;

pub fn process(_input: &str) -> usize {
    let mut input = parse_input(_input);
    input.sort_unstable_by_key(|(a, b, c)| (*c, *b, *a));

    let mut ans = 0;
    let mut bottom = vec![];
    let mut mid = vec![];
    let mut top = vec![];
    let mut z = input[0].2;

    for (a, b, c) in input {
        if c != z {
            let mut adj: FxHashSet<(isize, isize)> = FxHashSet::default();
            for (a, b) in &mid {
                adj.insert((a - 1, *b));
                adj.insert((a + 1, *b));
                adj.insert((*a, b - 1));
                adj.insert((*a, b + 1));
            }
            for (a, b) in adj {
                if !mid.contains(&(a, b))
                    && mid.contains(&(a - 1, b))
                    && mid.contains(&(a + 1, b))
                    && mid.contains(&(a, b - 1))
                    && mid.contains(&(a, b + 1))
                    && top.contains(&(a, b))
                    && bottom.contains(&(a, b))
                {
                    ans -= 6;
                }
            }
            bottom = mid;
            mid = top;
            top = vec![];
            z = c;
        }
        ans += 6;
        if mid.contains(&(a, b)) {
            ans -= 2
        }
        if top.contains(&(a - 1, b)) {
            ans -= 2
        }
        if top.contains(&(a, b - 1)) {
            ans -= 2
        }
        top.push((a, b));
    }

    let mut adj: FxHashSet<(isize, isize)> = FxHashSet::default();
    for (a, b) in &mid {
        adj.insert((a - 1, *b));
        adj.insert((a + 1, *b));
        adj.insert((*a, b - 1));
        adj.insert((*a, b + 1));
    }
    for (a, b) in adj {
        if !mid.contains(&(a, b))
            && mid.contains(&(a - 1, b))
            && mid.contains(&(a + 1, b))
            && mid.contains(&(a, b - 1))
            && mid.contains(&(a, b + 1))
            && top.contains(&(a, b))
            && bottom.contains(&(a, b))
        {
            ans -= 6;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 58);
    }
}
