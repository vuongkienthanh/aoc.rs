use crate::parsing::parse_input;
use crate::{build_image, variants};
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let rows = input.first().unwrap().1.rows();
    let cols = input.first().unwrap().1.cols();

    let image = build_image(input);
    let mut combined = vec![];

    for v in image {
        for row in 0..rows {
            let mut combined_row = vec![];
            for (_, grid) in &v {
                for cell in grid.iter_row(row) {
                    combined_row.push(*cell);
                }
            }
            combined.push(combined_row);
        }
    }
    let image = Grid::from(combined);

    let target = r#"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   "#;
    let target: Vec<(usize, usize)> = target
        .lines()
        .enumerate()
        .flat_map(|(row, line)| line.char_indices().map(move |(col, char)| (row, col, char)))
        .filter_map(|(row, col, c)| (c == '#').then_some((row, col)))
        .collect();
    let target_cols = target.iter().map(|(_, col)| col).max().cloned().unwrap() + 1;
    let target_rows = target.iter().map(|(row, _)| row).max().cloned().unwrap() + 1;


    for variant in variants(&image) {
        let mut count = 0;
        for row in 0..variant.rows() - target_rows+1 {
            for col in 0..variant.cols() - target_cols+1 {
                if target
                    .iter()
                    .map(|(r, c)| (r + row, c + col))
                    .all(|(r, c)| variant[(r, c)] == '#')
                {
                    count += 1;
                }
            }
        }
            println!("{count}");
    }

    0
}
