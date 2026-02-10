use crate::parsing::parse_input;
use fxhash::FxHashMap;

pub fn process(_input: &str) -> u64 {
    let input = parse_input(_input);
    let mut mem = FxHashMap::default();
    for (mask, v) in input {
        let (mut zero, mut one) = (0u64, 0u64);
        for c in mask.bytes() {
            match c {
                b'X' => {
                    zero <<= 1;
                    one <<= 1;
                }
                b'0' => {
                    zero = (zero << 1) | 1;
                    one <<= 1;
                }
                b'1' => {
                    zero <<= 1;
                    one = (one << 1) | 1;
                }
                _ => panic!(),
            }
        }
        for (loc, val) in v {
            mem.insert(loc as usize, ((val | zero) ^ zero) | one);
        }
    }
    mem.into_values().sum()
}
