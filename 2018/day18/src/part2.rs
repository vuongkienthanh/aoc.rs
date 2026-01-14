use crate::{next, parse, resource_value};
use fxhash::FxHashMap;

pub fn process(_input: &str) -> usize {
    let mut outskirts = parse(_input);
    let mut seen = FxHashMap::default();
    seen.insert(outskirts.clone(), 0);
    let mut i = 0;
    let rest = loop {
        i += 1;
        outskirts = next(outskirts);
        if let Some(j) = seen.insert(outskirts.clone(), i) {
            let diff = i - j;
            break (1_000_000_000 - i) % diff;
        }
    };
    for _ in 0..rest {
        outskirts = next(outskirts);
    }

    resource_value(outskirts)
}
