use crate::parsing::{F, parse_input};

pub fn process(_input: &str) -> usize {
    let (mut dots, folds) = parse_input(_input);
    let f = folds.into_iter().next().unwrap();
    let v: Vec<_> = match f {
        F::X(u) => dots
            .extract_if(|(x, _)| *x > u)
            .map(|(x, y)| (2 * u - x, y))
            .collect(),
        F::Y(u) => dots
            .extract_if(|(_, y)| *y > u)
            .map(|(x, y)| (x, 2 * u - y))
            .collect(),
    };
    dots.extend(v);
    dots.len()
}
