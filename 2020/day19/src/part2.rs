use crate::parsing::parse_input;
use crate::{Cache, find_set};

pub fn process(_input: &str) -> usize {
    let (map, msg) = parse_input(_input);
    let mut cache = Cache::default();
    let g31 = find_set(31, &map, &mut cache);
    let g42 = find_set(42, &map, &mut cache);

    // valid lines should consist of many 42s then many 31s which less than number of 42s

    msg.into_iter()
        .filter(|line| {
            let mut line = *line;
            let mut start_count = 0;
            'a: loop {
                for start in &g42 {
                    if line.starts_with(start) {
                        line = &line[start.len()..];
                        start_count += 1;
                        continue 'a;
                    }
                }
                break 'a;
            }
            if start_count == 0 {
                false
            } else {
                let mut end_count = 0;
                'a: loop {
                    for end in &g31 {
                        if line.starts_with(end) {
                            line = &line[end.len()..];
                            end_count += 1;
                            continue 'a;
                        }
                    }
                    break 'a;
                }
                line.is_empty() && end_count > 0 && start_count > end_count
            }
        })
        .count()
}
