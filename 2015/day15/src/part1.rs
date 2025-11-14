use crate::parsing::parse_input;
pub fn process(_input: &str) -> usize {
    let (_rest, input) = parse_input(_input).unwrap();
    assert!(_rest.is_empty());
    // todo!("part1")
    0
}

fn score(quantities: &Vec<isize>, input: &Vec<[isize; 5]>) -> usize {
    assert_eq!(quantities.len(), input.len());
    let property_scores = quantities
        .iter()
        .zip(input.iter())
        .fold([0; 4], |acc, (q, i)| {
            [
                acc[0] + q * i[0],
                acc[1] + q * i[1],
                acc[2] + q * i[2],
                acc[3] + q * i[3],
            ]
        });
    if property_scores.iter().any(|x| x <= &0) {
        0
    } else {
        property_scores.iter().product::<isize>() as usize
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#
    }
    #[rstest]
    fn test_score(fixture: &str) {
        let (_, input) = parse_input(fixture).unwrap();
        let quantities = vec![44, 56];
        assert_eq!(score(&quantities, &input), 62842880);
    }
    #[rstest]
    fn test_process(fixture: &str) {
        assert_eq!(process(fixture), 62842880);
    }
}
