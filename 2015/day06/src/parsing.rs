use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};

type Coord = (usize, usize);
#[derive(Debug, PartialEq)]
pub enum Action {
    On,
    Off,
    Toggle,
}
fn parse_coord(input: &str) -> IResult<&str, Coord> {
    separated_pair(complete::usize, tag(","), complete::usize).parse(input)
}
fn parse_line(input: &str) -> IResult<&str, (Action, Coord, Coord)> {
    let (input, action) = alt((
        tag("turn on ").map(|_| Action::On),
        tag("turn off ").map(|_| Action::Off),
        tag("toggle ").map(|_| Action::Toggle),
    ))
    .parse(input)?;
    let (input, (top_left, bottom_right)) =
        separated_pair(parse_coord, tag(" through "), parse_coord).parse(input)?;
    Ok((input, (action, top_left, bottom_right)))
}

pub fn parse_input(input: &str) -> Vec<(Action, Coord, Coord)> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("turn on 0,0 through 999,999", (Action::On, (0,0), (999,999)))]
    #[case("toggle 0,0 through 999,0", (Action::Toggle, (0,0), (999,0)))]
    #[case("turn off 499,499 through 500,500", (Action::Off, (499,499), (500,500)))]
    fn test_parse_line(#[case] input: &str, #[case] expected: (Action, Coord, Coord)) {
        let (_, line) = parse_line(input).unwrap();
        assert_eq!(line, expected);
    }
}
