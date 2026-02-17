use nom::{
    IResult, Parser,
    bytes::complete::take,
    character::complete::{self, anychar},
    combinator::all_consuming,
    multi::{count, many0, many1},
    sequence::terminated,
};

#[derive(Debug)]
pub struct Packet {
    pub version: u32,
    pub typeid: u8,
    pub literal: usize,
    pub subpackets: Vec<Packet>,
}
impl Packet {
    pub fn value(&self) -> usize {
        let v = &self.subpackets;
        match self.typeid {
            0 => v.iter().map(|x| x.value()).sum(),
            1 => v.iter().map(|x| x.value()).product(),
            2 => v.iter().map(|x| x.value()).min().unwrap(),
            3 => v.iter().map(|x| x.value()).max().unwrap(),
            4 => self.literal,
            5 => (v[0].value() > v[1].value()).into(),
            6 => (v[0].value() < v[1].value()).into(),
            7 => (v[0].value() == v[1].value()).into(),
            _ => panic!("unknown type id"),
        }
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    let (input, version) = take(3usize)
        .map(|x: &str| u32::from_str_radix(x, 2).unwrap())
        .parse(input)?;
    let (mut input, typeid) = take(3usize)
        .map(|x: &str| u8::from_str_radix(x, 2).unwrap())
        .parse(input)?;

    if typeid == 4 {
        let mut literal = 0;
        loop {
            let (i, group) = take(5usize)
                .map(|x: &str| usize::from_str_radix(x, 2).unwrap())
                .parse(input)?;
            input = i;
            literal = literal << 4 | (group & 0b1111);
            if group >> 4 == 0 {
                break;
            }
        }
        Ok((
            input,
            Packet {
                version,
                typeid,
                literal,
                subpackets: vec![],
            },
        ))
    } else {
        let (mut input, lengthtypeid) = anychar.parse(input)?;
        let subpackets = if lengthtypeid == '0' {
            let (i, bit15_len) = take(15usize)
                .map(|x: &str| usize::from_str_radix(x, 2).unwrap())
                .parse(input)?;
            let (i, bit15_s) = take(bit15_len).parse(i)?;
            let (_, subpackets) = all_consuming(many1(parse_packet)).parse(bit15_s)?;
            input = i;
            subpackets
        } else {
            let (i, n_packets) = take(11usize)
                .map(|x: &str| usize::from_str_radix(x, 2).unwrap())
                .parse(input)?;
            let (i, subpackets) = count(parse_packet, n_packets).parse(i)?;
            input = i;
            subpackets
        };
        Ok((
            input,
            Packet {
                version,
                typeid,
                literal: 0,
                subpackets,
            },
        ))
    }
}

pub fn parse_input(input: &str) -> Packet {
    let input: String = input
        .chars()
        .map(|x| match x {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!(),
        })
        .collect();
    all_consuming(terminated(parse_packet, many0(complete::char('0'))))
        .parse(input.as_str())
        .unwrap()
        .1
}
