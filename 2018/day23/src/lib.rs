pub mod parsing;
pub mod part1;
pub mod part2;
pub type Point = (isize, isize, isize);

fn manhattan((x,y,z): Point, (a,b,c): Point) -> usize {
    x.abs_diff(a) + y.abs_diff(b) + z.abs_diff(c)
}
