use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
};

type Aunt = (usize, [Option<usize>; 10]);
type Index = usize;

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse).parse(input)
}

fn parse_item(input: &str) -> IResult<&str, (Index, usize)> {
    (
        alt((
            tag("children: ").map(|_| 0),
            tag("cats: ").map(|_| 1),
            tag("samoyeds: ").map(|_| 2),
            tag("pomeranians: ").map(|_| 3),
            tag("akitas: ").map(|_| 4),
            tag("vizslas: ").map(|_| 5),
            tag("goldfish: ").map(|_| 6),
            tag("trees: ").map(|_| 7),
            tag("cars: ").map(|_| 8),
            tag("perfumes: ").map(|_| 9),
        )),
        parse_number,
    )
        .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Aunt> {
    (
        tag("Sue "),
        parse_number,
        tag(": "),
        parse_item,
        tag(", "),
        parse_item,
        tag(", "),
        parse_item,
    )
        .map(|(_, n, _, (a, x), _, (b, y), _, (c, z))| {
            let mut prop = [None; 10];
            prop[a] = Some(x);
            prop[b] = Some(y);
            prop[c] = Some(z);
            (n, prop)
        })
        .parse(input)
}
pub fn parse_input(input: &str) -> IResult<&str, Vec<Aunt>> {
    separated_list1(line_ending, parse_line).parse(input)
}
