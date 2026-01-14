use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::all_consuming,
    multi::{many0, many1, separated_list1},
    sequence::delimited,
};

#[derive(Debug)]
pub enum Dir {
    E,
    W,
    N,
    S,
}

#[derive(Debug)]
pub enum Item {
    Dirs(Vec<Dir>),
    Branch(Vec<Vec<Item>>),
}

fn parse_branch(input: &str) -> IResult<&str, Item> {
    delimited(tag("("), separated_list1(tag("|"), parse_items), tag(")"))
        .map(|v| Item::Branch(v))
        .parse(input)
}

fn parse_dirs(input: &str) -> IResult<&str, Item> {
    many1(alt((
        complete::char('E').map(|_| Dir::E),
        complete::char('W').map(|_| Dir::W),
        complete::char('N').map(|_| Dir::N),
        complete::char('S').map(|_| Dir::S),
    )))
    .map(|v| Item::Dirs(v))
    .parse(input)
}
fn parse_items(input: &str) -> IResult<&str, Vec<Item>> {
    many0(alt((parse_dirs, parse_branch))).parse(input)
}

fn parse_whole(input: &str) -> IResult<&str, Vec<Item>> {
    delimited(tag("^"), parse_items, tag("$")).parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(parse_whole).parse(input).unwrap().1
}