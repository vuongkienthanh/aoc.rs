use grid::Grid;

pub type Point = (usize, usize);

pub fn parse_input(input: &str) -> (Grid<u8>, Point, Point) {
    let (mut start, mut end) = ((0, 0), (0, 0));
    let mut grid = vec![];
    for (r, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (c, cell) in line.bytes().enumerate() {
            match cell {
                b'S' => {
                    row.push(b'a');
                    start = (r, c);
                }
                b'E' => {
                    row.push(b'z');
                    end = (r, c);
                }
                x => row.push(x),
            }
        }
        grid.push(row);
    }
    (Grid::from(grid), start, end)
}
