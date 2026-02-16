use crate::parsing::{F, parse_input};

pub fn process(_input: &str) -> usize {
    let (mut dots, folds) = parse_input(_input);
    for f in folds {
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
    }
    let (mut x_max, mut y_max) = (0, 0);
    for (x, y) in &dots {
        x_max = x_max.max(*x);
        y_max = y_max.max(*y);
    }

    for y in 0..=y_max {
        for x in 0..=x_max {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    0
}
