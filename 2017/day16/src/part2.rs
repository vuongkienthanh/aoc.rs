use crate::parsing::parse_input;
use crate::run;
use fxhash::FxHashMap as Map;

pub fn process(_input: &str) -> String {
    let input = parse_input(_input);
    let mut dancers: Vec<char> = ('a'..='p').collect();
    let mut map = Map::default();
    map.insert(dancers.clone(), 0);

    let mut found_loop = None;

    for i in 1..=1_000_000_000 {
        run(&mut dancers, &input);
        if let Some(a) = map.insert(dancers.clone(), i) {
            found_loop = Some((a, i));
            break;
        }
    }
    if let Some((a, b)) = found_loop {
        let mut rest = 1_000_000_000 - a;
        let diff = b - a;
        rest %= diff;
        for _ in 0..rest {
            run(&mut dancers, &input);
        }
    }

    dancers.into_iter().collect()
}
