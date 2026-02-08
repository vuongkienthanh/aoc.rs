use crate::{parse_input, Module, Pulse};
use std::collections::VecDeque;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

pub fn process(input: &'static str) -> usize {
    let mut module_map = parse_input(input);

    let final_layer = module_map
        .iter()
        .filter(|(_, v)| v.inner().dst().contains(&"rx".to_string()))
        .map(|(k, _)| *k)
        .collect::<Vec<_>>();
    assert_eq!(
        final_layer.len(),
        1,
        "Assumption #1: There is only 1 module pointing to rx"
    );
    assert!(
        match module_map.get(final_layer.get(0).unwrap()).unwrap() {
            Module::Conjunction(_) => true,
            _ => false,
        },
        "Assumption #2: The final module before rx is a conjunction"
    );

    let mut semi_final_layer = module_map
        .get(final_layer.get(0).unwrap())
        .unwrap()
        .inner()
        .src()
        .expect("Assumption #3: The modules on semi_final_layer send high pulses in regular intervals / cycles");

    let mut cycle_lengths: Vec<usize> = vec![];

    for button_push in 1usize.. {
        let mut stack =
            VecDeque::from([("button".to_string(), Pulse::Low, "broadcaster".to_string())]);
        while let Some((src, pulse, dst)) = stack.pop_front() {
            if let Some(module) = module_map.get_mut(dst.as_str()) {
                if let Some((new_pulse, new_dsts)) = module.inner_mut().send(&src, &pulse) {
                    for new_dst in new_dsts.into_iter() {
                        stack.push_back((dst.clone(), new_pulse.clone(), new_dst));
                    }
                    if semi_final_layer.contains(&dst) && new_pulse == Pulse::High {
                        semi_final_layer.retain_mut(|m| m != &dst);
                        cycle_lengths.push(button_push);
                    }
                }
            }
        }
        if semi_final_layer.is_empty() {
            break;
        }
    }

    lcm(&cycle_lengths)
}
