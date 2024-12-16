pub mod part1;
pub mod part2;

use nom::{
    bytes::complete::{tag, take, take_till, take_until},
    character::complete::line_ending,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};
type Coord = (isize, isize);

fn line(input: &str) -> IResult<&str, (Coord, Coord)> {
    separated_pair(coord, tag(" "), coord)(input)
}

fn coord(input: &str) -> IResult<&str, Coord> {
    preceded(
        take(2usize),
        separated_pair(
            take_until(",").map(|x: &str| x.parse::<isize>().unwrap()),
            tag(","),
            take_till(|c| ['\n', ' '].contains(&c)).map(|x: &str| x.parse::<isize>().unwrap()),
        )
        .map(|(y, x)| (x, y)),
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(Coord, Coord)>> {
    separated_list1(line_ending, line)(input)
}
