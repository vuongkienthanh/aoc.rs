use crate::parsing::parse_input;
use fxhash::FxHashMap as Map;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    detect(input)
}
fn detect(mut input: Vec<usize>) -> usize {
    let mut redistribution = 0;
    let mut seen = Map::default();
    seen.insert(input.clone(), 0);

    'a: loop {
        redistribution += 1;

        let (mut idx, mut blocks) = input
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|(_, ele)| *ele)
            .map(|(i, ele)| (i, *ele))
            .unwrap();
        input[idx] = 0;
        while blocks > 0 {
            blocks -= 1;
            idx = (idx + 1) % input.len();
            input[idx] += 1;
        }

        if let Some(r) = seen.insert(input.clone(), redistribution) {
            break 'a redistribution - r;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect() {
        assert_eq!(detect(vec![0, 2, 7, 0]), 4);
    }
}
