use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

type Aunt = (usize, [Option<usize>; 10]);
type Index = usize;

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
        complete::usize,
    )
        .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Aunt> {
    (
        tag("Sue "),
        complete::usize,
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
pub fn parse_input(input: &str) -> Vec<Aunt> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
