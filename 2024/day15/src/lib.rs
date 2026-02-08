pub mod part1;
pub mod part2;
use grid::Grid;

#[derive(Debug, Eq, PartialEq, Clone)]
enum CellType {
    SBox,
    DBox(Side),
    Empty,
    Wall,
}
#[derive(Debug, Eq, PartialEq, Clone)]
enum Side {
    Left,
    Right,
}

#[derive(Debug)]
enum MoveType {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct MyBox {
    left: Coord,
    right: Coord,
}

type Coord = (usize, usize);

fn print_grid(robot: Coord, grid: &Grid<CellType>) {
    for (i, row) in grid.iter_rows().enumerate() {
        for (j, c) in row.enumerate() {
            if (i, j) == robot {
                print!("@");
            } else {
                match c {
                    CellType::DBox(Side::Left) => print!("["),
                    CellType::DBox(Side::Right) => print!("]"),
                    CellType::Empty => print!("."),
                    CellType::Wall => print!("#"),
                    CellType::SBox => print!("O"),
                }
            }
        }
        println!();
    }
}

fn parse_instruction(input: &str) -> Vec<MoveType> {
    input
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '^' => MoveType::Up,
                'v' => MoveType::Down,
                '>' => MoveType::Right,
                '<' => MoveType::Left,
                _ => panic!("unknown move type"),
            })
        })
        .collect()
}
