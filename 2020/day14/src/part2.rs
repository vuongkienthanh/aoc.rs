use crate::parsing::parse_input;
use fxhash::FxHashMap;

pub fn process(_input: &str) -> u64 {
    let input = parse_input(_input);
    let mut mem = FxHashMap::default();
    for (mask, v) in input {
        let mut masks = vec![0u64];
        let mut xs = 0u64;
        for c in mask.bytes() {
            match c {
                b'X' => {
                    for m in masks.iter_mut() {
                        *m <<= 1;
                    }
                    let mut new_masks = masks.clone();
                    for m in new_masks.iter_mut() {
                        *m |= 1;
                    }
                    masks.extend(new_masks);
                    xs = (xs << 1) | 1;
                }
                b'0' => {
                    for m in masks.iter_mut() {
                        *m <<= 1;
                    }
                    xs <<= 1;
                }
                b'1' => {
                    for m in masks.iter_mut() {
                        *m = (*m << 1) | 1;
                    }
                    xs <<= 1;
                }
                _ => panic!(),
            }
        }
        for (mut loc, val) in v {
            loc = (loc | xs) ^ xs;
            for m in &masks {
                mem.insert(loc | m, val);
            }
        }
    }
    mem.into_values().sum()
}
