pub mod part1;
pub mod part2;

type Coord = (usize, usize);

fn is_wall(input: usize, coord: Coord) -> bool {
    let n = (coord.0 + coord.1).pow(2u32) + 3 * coord.0 + coord.1 + input;
    !n.count_ones().is_multiple_of(2)
}
