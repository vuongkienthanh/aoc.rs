use crate::{parse_input, Pulse};
use std::collections::VecDeque;
pub fn process(input: &'static str) -> usize {
    let mut module_map = parse_input(input);
    let mut loop_len = 0;
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for button_push in 0..1000 {
        let mut stack: VecDeque<(String, Pulse, String)> =
            VecDeque::from([("button".to_string(), Pulse::Low, "broadcaster".to_string())]);
        low_pulses += 1;
        while let Some((src, pulse, dst)) = stack.pop_front() {
            // dbg!((&src, &pulse, &dst));
            if let Some(module) = module_map.get_mut(dst.as_str()) {
                if let Some((new_pulse, new_dsts)) = module.inner_mut().send(&src, &pulse) {
                    for new_dst in new_dsts.into_iter() {
                        stack.push_back((dst.clone(), new_pulse.clone(), new_dst));
                        match new_pulse {
                            Pulse::Low => low_pulses += 1,
                            Pulse::High => high_pulses += 1,
                        }
                    }
                }
            }
        }
        if module_map.values().all(|m| m.inner().is_backed_to_origin()) {
            loop_len = button_push + 1;
            break;
        }
    }
    let remaining = match loop_len {
        0 => 0,
        l => {
            let number_of_loops = 1000usize.div_euclid(loop_len);
            low_pulses *= number_of_loops;
            high_pulses *= number_of_loops;
            1000usize.rem_euclid(l)
        }
    };

    for _ in 0..remaining {
        let mut stack: VecDeque<(String, Pulse, String)> =
            VecDeque::from([("button".to_string(), Pulse::Low, "broadcaster".to_string())]);
        low_pulses += 1;
        while let Some((src, pulse, dst)) = stack.pop_front() {
            // dbg!((&src, &pulse, &dst));
            if let Some(module) = module_map.get_mut(dst.as_str()) {
                if let Some((new_pulse, new_dsts)) = module.inner_mut().send(&src, &pulse) {
                    for new_dst in new_dsts.into_iter() {
                        stack.push_back((dst.clone(), new_pulse.clone(), new_dst));
                        match new_pulse {
                            Pulse::Low => low_pulses += 1,
                            Pulse::High => high_pulses += 1,
                        }
                    }
                }
            }
        }
    }
    // dbg!(&parsed);
    // dbg!(&parsed.values().all(|m| m.is_backed_to_origin()));
    // dbg!(loop_len);
    low_pulses * high_pulses
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_1a() {
        let input = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;
        assert_eq!(process(input), 32000000);
    }
    #[test]
    fn test_process_1b() {
        let input = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;
        assert_eq!(process(input), 11687500);
    }
}
