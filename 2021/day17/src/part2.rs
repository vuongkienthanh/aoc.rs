use crate::parsing::parse_input;
use fxhash::FxHashSet;

pub fn process(_input: &str) -> usize {
    let (min_x, max_x, max_y, min_y) = parse_input(_input);
    let mut ans = FxHashSet::default();
    for vy in max_y..-max_y {
        for step in 1.. {
            let dy = (2 * vy + 1 - step) * step / 2;
            if dy > min_y {
                continue;
            }
            if dy < max_y {
                break;
            }

            for vx in 1.. {
                let dx = (2 * vx + 1 - step.min(vx)) * step.min(vx) / 2;
                if dx < min_x {
                    continue;
                }
                if dx > max_x {
                    break;
                }
                ans.insert((vx, vy));
            }
        }
    }

    ans.len()
}
