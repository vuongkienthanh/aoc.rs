use crate::parsing::parse_input;
use crate::valid_ranges;

pub fn process(_input: &str) -> usize {
    let (class, your, mut nearby) = parse_input(_input);
    let valid_ranges = valid_ranges(&class);
    nearby.retain(|v| {
        v.iter()
            .all(|x| valid_ranges.iter().all(|(a, b)| x >= a && x <= b))
    });

    let mut possibilities = vec![(0..class.len()).collect::<Vec<_>>(); class.len()];

    for ticket in nearby {
        for i in 0..class.len() {
            let val = ticket[i];
            let mut new_possible = vec![];

            for possible in possibilities.get(i).unwrap() {
                let (_name, ((a, b), (c, d))) = class.get(*possible).unwrap();
                if (val >= *a && val <= *b) || (val >= *c && val <= *d) {
                    new_possible.push(*possible);
                }
            }

            *possibilities.get_mut(i).unwrap() = new_possible;
        }
    }
    while possibilities.iter().any(|v| v.len() > 1) {
        let ones: Vec<usize> = possibilities
            .iter()
            .filter(|x| x.len() == 1)
            .flatten()
            .cloned()
            .collect();
        for p in possibilities.iter_mut() {
            if p.len() == 1 {
                continue;
            }
            p.retain(|x| !ones.contains(x));
        }
    }
    class
        .into_iter()
        .enumerate()
        .filter_map(|(i, (x, _))| x.starts_with("departure").then_some(i))
        .map(|i| {
            possibilities
                .iter()
                .flatten()
                .enumerate()
                .find_map(|(j, x)| (*x == i).then_some(j))
                .unwrap()
        })
        .map(|i| your[i])
        .product()
}
