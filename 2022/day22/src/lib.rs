pub mod parsing;
pub mod part1;
pub mod part2;
use grid::Grid;
use parsing::Cell;

// since i dont know how to fold 3D
// i just hardcode the mapping
// the input is shaped
//   0 1
//   2
// 3 4
// 5
//
// face (up, down, left, right)
// 0 (5 left, 2 up, 3 left -, 1 left)
// 1 (5 down, 2 right, 0 right, 4 right -)
// 2 (0 down, 4 up, 3 up, 1 down)
// 3 (2 left, 5 up, 0 left -, 4 left)
// 4 (2 down, 5 right, 3 right, 1 right -)
// 5 (3 down, 1 up, 0 up, 4 down)
//

pub enum Border {
    Up,
    Down,
    Left,
    Right,
}

pub const FACES: [[(usize, Border); 4]; 6] = [
    [
        (5, Border::Left),
        (2, Border::Up),
        (3, Border::Left),
        (1, Border::Left),
    ],
    [
        (5, Border::Down),
        (2, Border::Right),
        (0, Border::Right),
        (4, Border::Right),
    ],
    [
        (0, Border::Down),
        (4, Border::Up),
        (3, Border::Up),
        (1, Border::Down),
    ],
    [
        (2, Border::Left),
        (5, Border::Up),
        (0, Border::Left),
        (4, Border::Left),
    ],
    [
        (2, Border::Down),
        (5, Border::Right),
        (3, Border::Right),
        (1, Border::Right),
    ],
    [
        (3, Border::Down),
        (1, Border::Up),
        (0, Border::Up),
        (4, Border::Down),
    ],
];

pub fn build_cube(map: Vec<Vec<Cell>>) -> [Grid<Cell>; 6] {
    [
        Grid::from(
            map.iter()
                .take(50)
                .map(|row| row.iter().skip(50).take(50).cloned().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        ),
        Grid::from(
            map.iter()
                .take(50)
                .map(|row| row.iter().skip(100).take(50).cloned().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        ),
        Grid::from(
            map.iter()
                .skip(50)
                .take(50)
                .map(|row| row.iter().skip(50).take(50).cloned().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        ),
        Grid::from(
            map.iter()
                .skip(100)
                .take(50)
                .map(|row| row.iter().take(50).cloned().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        ),
        Grid::from(
            map.iter()
                .skip(100)
                .take(50)
                .map(|row| row.iter().skip(50).take(50).cloned().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        ),
        Grid::from(
            map.iter()
                .skip(150)
                .take(50)
                .map(|row| row.iter().take(50).cloned().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        ),
    ]
}
