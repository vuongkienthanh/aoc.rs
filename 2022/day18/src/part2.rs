use crate::naive_surface;
use crate::parsing::parse_input;
use std::collections::BTreeSet;

pub fn process(_input: &str) -> usize {
    let input = BTreeSet::from_iter(parse_input(_input).into_iter().map(|(x, y, z)| (z, y, x)));
    let mut min_x = i8::MAX;
    let mut max_x = 0;
    let mut min_y = i8::MAX;
    let mut max_y = 0;
    let mut min_z = i8::MAX;
    let mut max_z = 0;
    for (z, y, x) in &input {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
        min_z = min_z.min(*z);
        max_z = max_z.max(*z);
    }
    println!("{min_x} {max_x} {min_y} {max_y} {min_z} {max_z}");

    // for (z, y, x) in &input {
    //     println!("{:?}", (x, y, z));
    // }

    let mut ans = 0;
    let mut bottom = vec![];
    let mut top = vec![];
    let mut cz = input.first().unwrap().0;
    let mut cy = input.first().unwrap().1;
    let mut adj = vec![];

    for (z, y, x) in &input {
        // if new_z, pop_old_y last point
        // also update y
        if *z != cz {
            bottom = std::mem::take(&mut top);
            cz = *z;
            adj.pop().unwrap();
            cy = *y;
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

        // if new_y, pop old_y last point
        // nothing happens when new_z because y is already updated
        if *y != cy {
            adj.pop().unwrap();
            cy = *y;
        }

        // push adj right
        adj.push((*z, *y, *x + 1));
    }
    // pop last adj on last y
    adj.pop().unwrap();

    // keep those aren't droplet
    adj.retain(|item| !input.contains(item));

    // expand air pockets using BFS
    let mut checked = BTreeSet::new();
    'adj: for item in adj {
        if checked.contains(&item) {
            continue 'adj;
        }
        let mut pocket = BTreeSet::new();
        let mut current = vec![item];
        while !current.is_empty() {
            let mut new = vec![];
            for item in current {
                // leaked outside bound
                if item.0 < min_z
                    || item.0 > max_z
                    || item.1 < min_y
                    || item.1 > max_y
                    || item.2 < min_x
                    || item.2 > max_x
                {
                    checked.extend(pocket);
                    continue 'adj;
                }
                if !input.contains(&item) && pocket.insert(item) {
                    new.push((item.0 - 1, item.1, item.2));
                    new.push((item.0 + 1, item.1, item.2));
                    new.push((item.0, item.1 - 1, item.2));
                    new.push((item.0, item.1 + 1, item.2));
                    new.push((item.0, item.1, item.2 - 1));
                    new.push((item.0, item.1, item.2 + 1));
                }
            }
            current = new;
        }
        ans -= naive_surface(&pocket);
        checked.extend(pocket);
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
