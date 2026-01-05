use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::{many1, separated_list1},
};
use nom_locate::LocatedSpan;
type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Eq, PartialEq)]
pub enum Item {
    Wall,
    Space,
    Goblin,
    Elf,
}

fn parse_item(input: Span) -> IResult<Span, (Item, Option<(usize, usize)>)> {
    let loc = (input.location_line() as usize, input.get_column());
    alt((
        complete::char('#').map(|_| (Item::Wall, None)),
        complete::char('.').map(|_| (Item::Space, None)),
        complete::char('G').map(|_| (Item::Goblin, Some(loc))),
        complete::char('E').map(|_| (Item::Elf, Some(loc))),
    ))
    .parse(input)
}

fn parse_line(input: Span) -> IResult<Span, (Vec<Item>, Vec<(usize, usize)>)> {
    let (input, line) = many1(parse_item).parse(input)?;
    let (line, units) =
        line.into_iter()
            .fold((vec![], vec![]), |(mut line, mut units), (item, loc)| {
                match item {
                    Item::Wall | Item::Space => line.push(item),
                    Item::Elf | Item::Goblin => {
                        line.push(item);
                        units.push(loc.unwrap());
                    }
                }
                (line, units)
            });
    Ok((input, (line, units)))
}

pub fn parse_input(input: &str) -> (Vec<Vec<Item>>, Vec<((usize, usize), usize)>) {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(Span::new(input))
        .unwrap()
        .1
        .into_iter()
        .fold(
            (vec![], vec![]),
            |(mut map, mut units), (line, line_units)| {
                map.push(line);
                for unit in line_units {
                    units.push((unit, 200));
                }
                (map, units)
            },
        )
}
