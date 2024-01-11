use crate::{parse_input, Pulse};
use std::collections::VecDeque;
pub fn process(input: &'static str) -> usize {
    let mut parsed = parse_input(input);
    for i in 0.. {
        let mut rx_low = 0;
        let mut stack: VecDeque<(String, Pulse, String)> =
            VecDeque::from([("button".to_string(), Pulse::Low, "broadcaster".to_string())]);
        while let Some((src, pulse, dst)) = stack.pop_front() {
            // dbg!((&src, &pulse, &dst));
            if let Some(module) = parsed.get_mut(dst.as_str()) {
                if let Some((new_pulse, new_dsts)) = module.send(&src, &pulse) {
                    for new_dst in new_dsts.into_iter() {
                        match (&new_pulse, new_dst.as_str()) {
                            (&Pulse::Low, "rx") => rx_low += 1,
                            _ => (),
                        };
                        stack.push_back((dst.clone(), new_pulse.clone(), new_dst));
                    }
                }
            }
        }
        if rx_low == 1 {
            return i + 1;
        }
    }
    unreachable!("should loop forever")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#""#;
        assert_eq!(process(input), 0);
    }
}
