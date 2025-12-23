use crate::parsing::parse_input;
use grid::Grid;

type Point = (usize, usize);

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let (x, y, _, _) = input.last().cloned().unwrap();
    let rows = x + 1;
    let cols = y + 1;

    // inspect the map
    // from part 1, you know that any box can't hold 2 data at a time
    // on the map you will see a wall
    // let mut i = input.iter();
    // for x in 0..rows {
    //     for y in 0..cols {
    //         let node = i.next().unwrap();
    //         if node.2 > 100 {
    //             print!("0"); // wall box
    //         } else if node.2 == 0 {
    //             print!("x"); // empty box
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!();
    // }

    let mut grid: Grid<usize> = Grid::new(rows, cols);
    for item in input {
        grid[(item.0, item.1)] = item.2;
    }

    let empty: Point = grid
        .iter_rows()
        .enumerate()
        .find_map(|(i, row)| {
            row.enumerate()
                .find_map(|(j, used)| (*used == 0).then_some((i, j)))
        })
        .unwrap();

    let wall_edge: Point = grid
        .iter_rows()
        .enumerate()
        .find_map(|(i, row)| {
            row.enumerate()
                .find_map(|(j, used)| (*used > 300).then(|| (i - 1, j)))
        })
        .unwrap();

    let target: Point = (rows - 1, 0);

    // answer is the distance from empty box to wall_edge then to target
    // for each space between target (already 1 step closer) and (0,0)
    // you have to move 5 times to make it 1 step closer
    manhattan_distance(empty, wall_edge)
        + manhattan_distance(wall_edge, target)
        + 5 * (manhattan_distance(target, (0, 0)) - 1)
}

fn manhattan_distance(a: Point, b: Point) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}
