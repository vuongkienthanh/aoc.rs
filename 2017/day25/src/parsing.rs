use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded},
};

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}
#[derive(Debug)]
pub struct Instruction {
    pub write: usize,
    pub mov: Direction,
    pub to: usize,
}

#[derive(Debug)]
pub struct State {
    pub if0: Instruction,
    pub if1: Instruction,
}

fn parse_state_name(input: &str) -> IResult<&str, usize> {
    preceded(tag("state "), complete::anychar)
        .map(|x| x as u8 - b'A')
        .map(|x| x as usize)
        .parse(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    (
        delimited(
            (space1, tag("- Write the value ")),
            complete::usize,
            tag("."),
        ),
        line_ending,
        delimited(
            (space1, tag("- Move one slot to the ")),
            alt((
                tag("right").map(|_| Direction::Right),
                tag("left").map(|_| Direction::Left),
            )),
            tag("."),
        ),
        line_ending,
        delimited(
            (space1, tag("- Continue with ")),
            parse_state_name,
            tag("."),
        ),
    )
        .map(|(write, _, mov, _, to)| Instruction { write, mov, to })
        .parse(input)
}

fn parse_state(input: &str) -> IResult<&str, State> {
    (
        (tag("In "), parse_state_name, tag(":"), line_ending),
        (space1, tag("If the current value is 0:"), line_ending),
        parse_instruction,
        line_ending,
        (space1, tag("If the current value is 1:"), line_ending),
        parse_instruction,
    )
        .map(|(_, _, if0, _, _, if1)| State { if0, if1 })
        .parse(input)
}

fn parse_begin(input: &str) -> IResult<&str, (usize, usize)> {
    (
        delimited(tag("Begin in "), parse_state_name, tag(".")),
        line_ending,
        delimited(
            tag("Perform a diagnostic checksum after "),
            complete::usize,
            tag(" steps."),
        ),
        (line_ending, line_ending),
    )
        .map(|(a, _, b, _)| (a, b))
        .parse(input)
}
fn parse_states(input: &str) -> IResult<&str, Vec<State>> {
    separated_list1((line_ending, line_ending), parse_state).parse(input)
}

pub fn parse_input(input: &str) -> ((usize, usize), Vec<State>) {
    all_consuming((parse_begin, parse_states))
        .parse(input)
        .unwrap()
        .1
}
