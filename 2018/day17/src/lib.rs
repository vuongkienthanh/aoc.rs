pub mod parsing;
pub mod part1;
pub mod part2;
use parsing::Item;

pub type Point = (usize, usize);
pub type Map = Vec<Vec<char>>;

fn build_map(input: Vec<Item>) -> (Map, usize, usize) {
    let (min_x, max_x, min_y, max_y) = input.iter().fold(
        (usize::MAX, 0, usize::MAX, 0),
        |(min_x, max_x, min_y, max_y), item| match item {
            Item::X(x, (a, b)) => (min_x.min(*x), max_x.max(*x), min_y.min(*a), max_y.max(*b)),
            Item::Y(y, (a, b)) => (min_x.min(*a), max_x.max(*b), min_y.min(*y), max_y.max(*y)),
        },
    );
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; max_x - min_x + 3]; max_y + 1];
    for item in input {
        match item {
            Item::X(x, (a, b)) => {
                for y in a..=b {
                    grid[y][x - min_x + 1] = '#';
                }
            }
            Item::Y(y, (a, b)) => {
                for x in a..=b {
                    grid[y][x - min_x + 1] = '#';
                }
            }
        }
    }
    (grid, min_x, min_y)
}

fn display_map(map: &Map) {
    for row in map {
        for cell in row {
            print!("{cell}");
        }
        println!()
    }
}

fn find_base(spring: Point, map: &mut Map) -> Option<usize> {
    for y in spring.1 + 1..map.len() {
        match map[y][spring.0] {
            '.' => {
                map[y][spring.0] = '|';
            }
            '#' | '~' => return Some(y),
            '|' => return None,
            _ => panic!(),
        }
    }
    None
}

fn fill_row(spring: Point, y: usize, map: &mut Map) -> Vec<Point> {
    #[derive(Debug)]
    enum Meet {
        Wall(usize),
        Spring(usize),
        AlreadyFilled,
    }
    let width = map.get(0).unwrap().len();
    let left = (0..spring.0)
        .rev()
        .find_map(|x| match map[y][x] {
            '~' => Some(Meet::AlreadyFilled),
            '.' | '|' => match map[y + 1][x] {
                '#' | '~' => None,
                '.' | '|' => Some(Meet::Spring(x)),
                _ => panic!(),
            },
            '#' => Some(Meet::Wall(x)),
            _ => panic!(),
        })
        .unwrap();
    let right = (spring.0..width)
        .find_map(|x| match map[y][x] {
            '~' => Some(Meet::AlreadyFilled),
            '.' | '|' => match map[y + 1][x] {
                '#' | '~' => None,
                '.' | '|' => Some(Meet::Spring(x)),
                _ => panic!(),
            },
            '#' => Some(Meet::Wall(x)),
            _ => panic!(),
        })
        .unwrap();
    match (left, right) {
        (Meet::AlreadyFilled, _) | (_, Meet::AlreadyFilled) => {
            vec![]
        }
        (Meet::Wall(a), Meet::Wall(b)) => {
            for x in a + 1..b {
                map[y][x] = '~';
            }
            vec![]
        }
        (Meet::Spring(a), Meet::Wall(b)) => {
            for x in a..b {
                map[y][x] = '|';
            }
            vec![(a, y)]
        }
        (Meet::Wall(a), Meet::Spring(b)) => {
            for x in a + 1..=b {
                map[y][x] = '|';
            }
            vec![(b, y)]
        }
        (Meet::Spring(a), Meet::Spring(b)) => {
            for x in a..=b {
                map[y][x] = '|';
            }
            vec![(a, y), (b, y)]
        }
    }
}
