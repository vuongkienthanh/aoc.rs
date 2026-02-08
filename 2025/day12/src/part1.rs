use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (blocks, input) = parse_input(_input);

    input
        .into_iter()
        .enumerate()
        .filter(|(i, (size, pieces))| {
            let area = size.0 * size.1;
            let needed_area = pieces
                .iter()
                .zip(blocks.iter())
                .map(|(a, b)| a * *b)
                .sum();
            println!("puzzle {i} = {area} when need {needed_area}");
            area >= needed_area
        })
        .count()
}
