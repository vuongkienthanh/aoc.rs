pub mod parsing;
pub mod part1;
pub mod part2;

use grid::Grid;
use std::collections::VecDeque;

fn variants(grid: &Grid<char>) -> Vec<Grid<char>> {
    let mut ans = vec![grid.clone()];
    let mut grid = grid.clone();
    for _ in 0..3 {
        grid.rotate_left();
        ans.push(grid.clone());
    }
    let mut grid = grid.clone();
    grid.flip_cols();
    ans.push(grid.clone());
    for _ in 0..3 {
        grid.rotate_left();
        ans.push(grid.clone());
    }
    ans
}

fn build_image(mut input: Vec<(usize, Grid<char>)>) -> Vec<Vec<(usize, Grid<char>)>> {
    let rows = input.first().unwrap().1.rows();
    let cols = input.first().unwrap().1.cols();

    let mut base_line = VecDeque::new();
    base_line.push_back(input.pop().unwrap());

    while let Some((i, grid)) = right(&base_line.back().unwrap().1, cols, &mut input) {
        base_line.push_back((i, grid));
    }

    while let Some((i, grid)) = left(&base_line.front().unwrap().1, cols, &mut input) {
        base_line.push_front((i, grid));
    }

    let mut image: VecDeque<Vec<(usize, Grid<char>)>> = VecDeque::new();
    image.push_back(base_line.into_iter().collect());

    while let Some((i, grid)) = up(
        &(image.front().unwrap().get(0).unwrap().1),
        rows,
        &mut input,
    ) {
        let mut row = vec![(i, grid)];
        while let Some((i, grid)) = right(&(row.last().unwrap().1), cols, &mut input) {
            row.push((i, grid));
        }
        image.push_front(row);
    }
    while let Some((i, grid)) = down(&(image.back().unwrap().get(0).unwrap().1), rows, &mut input) {
        let mut row = vec![(i, grid)];
        while let Some((i, grid)) = right(&(row.last().unwrap().1), cols, &mut input) {
            row.push((i, grid));
        }
        image.push_back(row);
    }

    // println!("printing whole image");
    // for line in &image {
    //     for (i, grid) in line {
    //         print!("{i}       ");
    //     }
    //     println!();
    //     for row in 0..rows {
    //         for (_, grid) in line {
    //             for c in grid.iter_row(row) {
    //                 print!("{c}");
    //             }
    //             print!(" ");
    //         }
    //         println!();
    //     }
    // }

    image.into_iter().collect()
}

fn right(
    grid: &Grid<char>,
    cols: usize,
    input: &mut Vec<(usize, Grid<char>)>,
) -> Option<(usize, Grid<char>)> {
    let side: Vec<&char> = grid.iter_col(cols - 1).collect();
    let mut new_input = vec![];
    let mut ans = None;
    'a: for _ in 0..input.len() {
        let (i, grid) = input.pop().unwrap();
        for variant in variants(&grid) {
            if variant.iter_col(0).collect::<Vec<_>>() == side {
                ans = Some((i, variant));
                break 'a;
            }
        }
        new_input.push((i, grid));
    }
    while let Some((i, grid)) = input.pop() {
        new_input.push((i, grid));
    }
    *input = new_input;
    ans
}

fn left(
    grid: &Grid<char>,
    cols: usize,
    input: &mut Vec<(usize, Grid<char>)>,
) -> Option<(usize, Grid<char>)> {
    let side: Vec<&char> = grid.iter_col(0).collect();
    let mut new_input = vec![];
    let mut ans = None;
    'a: for _ in 0..input.len() {
        let (i, grid) = input.pop().unwrap();
        for variant in variants(&grid) {
            if variant.iter_col(cols - 1).collect::<Vec<_>>() == side {
                ans = Some((i, variant));
                break 'a;
            }
        }
        new_input.push((i, grid));
    }
    while let Some((i, grid)) = input.pop() {
        new_input.push((i, grid));
    }
    *input = new_input;
    ans
}

fn up(
    grid: &Grid<char>,
    rows: usize,
    input: &mut Vec<(usize, Grid<char>)>,
) -> Option<(usize, Grid<char>)> {
    let side: Vec<&char> = grid.iter_row(0).collect();
    let mut new_input = vec![];
    let mut ans = None;
    'a: for _ in 0..input.len() {
        let (i, grid) = input.pop().unwrap();
        for variant in variants(&grid) {
            if variant.iter_row(rows - 1).collect::<Vec<_>>() == side {
                ans = Some((i, variant));
                break 'a;
            }
        }
        new_input.push((i, grid));
    }
    while let Some((i, grid)) = input.pop() {
        new_input.push((i, grid));
    }
    *input = new_input;
    ans
}

fn down(
    grid: &Grid<char>,
    rows: usize,
    input: &mut Vec<(usize, Grid<char>)>,
) -> Option<(usize, Grid<char>)> {
    let side: Vec<&char> = grid.iter_row(rows - 1).collect();
    let mut new_input = vec![];
    let mut ans = None;
    'a: for _ in 0..input.len() {
        let (i, grid) = input.pop().unwrap();
        for variant in variants(&grid) {
            if variant.iter_row(0).collect::<Vec<_>>() == side {
                ans = Some((i, variant));
                break 'a;
            }
        }
        new_input.push((i, grid));
    }
    while let Some((i, grid)) = input.pop() {
        new_input.push((i, grid));
    }
    *input = new_input;
    ans
}
