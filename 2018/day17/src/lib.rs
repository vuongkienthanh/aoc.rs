pub mod parsing;
pub mod part1;
pub mod part2;
use aoc_helper::range::merge;
use parsing::Item;

pub type Point = (usize, usize);

fn build_map(input: Vec<Item>) -> (Vec<Vec<char>>, usize) {
    let (min_x, max_x, max_y) = input.iter().fold(
        (usize::MAX, 0, 0),
        |(min_x, max_x, max_y), item| match item {
            Item::X(x, (_, b)) => (min_x.min(*x), max_x.max(*x), max_y.max(*b)),
            Item::Y(y, (a, b)) => (min_x.min(*a), max_x.max(*b), max_y.max(*y)),
        },
    );
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; max_x - min_x + 1]; max_y + 1];
    for item in input {
        match item {
            Item::X(x, (a, b)) => {
                for y in a..=b {
                    grid[y][x - min_x] = '#';
                }
            }
            Item::Y(y, (a, b)) => {
                for x in a..=b {
                    grid[y][x - min_x] = '#';
                }
            }
        }
    }
    (grid, min_x)
}
fn display_map(map: &[Vec<char>]) {
    for row in map {
        for cell in row {
            print!("{cell}");
        }
        println!("")
    }
}

fn find_base(spring: Point, map: &mut [Vec<char>]) -> Option<usize> {
    if let Some(base) = (spring.1 + 1..map.len()).find(|y| map[*y][spring.0] == '#') {
        for y2 in spring.1 + 1..base {
            map[y2][spring.0] = '|';
        }
        Some(base)
    } else {
        for y2 in spring.1 + 1..map.len() {
            map[y2][spring.0] = '|';
        }
        None
    }
}

fn fill_row(spring: Point, y: usize, map: &Map) -> (Option<usize>, Option<usize>) {
    let width = map.get().unwrap().len();
    let left_wall = (0..spring.0).rev().find(|x| map[y][*x] == '#');
    let right_wall = (spring.0 + 1..width).find(|x| map[y][*x] == '#');
    let left_base = (0..spring.0)
        .rev()
        .take_while(|x| ['#', '~'].contains(&map[y + 1][*x])).last().unwrap();
    let right_base = (spring.0 + 1..width)
        .rev()
        .take_while(|x| ['#', '~'].contains(&map[y + 1][*x])).last().unwrap();
}
