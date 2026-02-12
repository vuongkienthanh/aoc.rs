use crate::parsing::parse_input;
use crate::{Cache, find_set};

pub fn process(_input: &str) -> usize {
    let (map, msg) = parse_input(_input);
    let mut cache = Cache::default();
    let g31 = find_set(31, &map, &mut cache);
    let g42 = find_set(42, &map, &mut cache);

    // g42 should not intersect with g31
    // println!(
    //     "g42.len={} g31.len={} intersection={:?}",
    //     g42.len(),
    //     g31.len(),
    //     g42.intersection(&g31)
    // );
    // valid lines should consist of (42, 42, 31)

    msg.into_iter()
        .filter(|line| {
            let mut line = *line;
            let mut is_valid = false;
            'a: for start in &g42 {
                if line.starts_with(start) {
                    line = &line[start.len()..];
                    is_valid = true;
                    break 'a;
                }
            }
            if is_valid {
                is_valid = false;
                'a: for start in &g42 {
                    if line.starts_with(start) {
                        line = &line[start.len()..];
                        is_valid = true;
                        break 'a;
                    }
                }
            }
            if is_valid {
                is_valid = false;
                'a: for end in &g31 {
                    if line.starts_with(end) {
                        line = &line[end.len()..];
                        is_valid = true;
                        break 'a;
                    }
                }
            }
            line.is_empty() && is_valid
        })
        .count()
}
