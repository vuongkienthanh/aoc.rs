use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::delimited,
};
#[derive(Debug)]
pub enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
pub enum Item {
    Unit(usize),
    Group { nums: Vec<Item>, ops: Vec<Op> },
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    alt((tag(" + ").map(|_| Op::Add), tag(" * ").map(|_| Op::Mul))).parse(input)
}
fn parse_inner_group(input: &str) -> IResult<&str, Item> {
    let (mut input, item) = parse_item(input)?;
    let mut nums = vec![item];
    let mut ops = vec![];
    loop {
        if let Ok((i, op)) = parse_op(input) {
            input = i;
            ops.push(op);
        } else {
            break;
        }
        if let Ok((i, item)) = parse_item(input) {
            input = i;
            nums.push(item);
        } else {
            panic!("should have item after op");
        }
    }
    Ok((input, Item::Group { nums, ops }))
}
fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((
        delimited(tag("("), parse_inner_group, tag(")")),
        complete::usize.map(|x| Item::Unit(x)),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_inner_group))
        .parse(input)
        .unwrap()
        .1
}
