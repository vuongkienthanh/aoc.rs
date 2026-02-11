use fxhash::FxHashMap;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, anychar, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
};

#[derive(Debug, Clone)]
pub enum Rule {
    Char(char),
    Or(Vec<Vec<usize>>),
}

impl Rule {
    fn is_matched<'a>(
        &'a self,
        mut msg: &'a str,
        rules: &'a FxHashMap<usize, Rule>,
    ) -> Option<&'a str> {
        if msg.is_empty() {
            return None;
        }
        match self {
            Rule::Char(c) => (msg.chars().next().unwrap() == *c).then_some(&msg[1..]),
            Rule::Or(v) => {
                let mut ans = None;
                'a: for chain in v {
                    let mut msg = msg.clone();
                    for rule in chain {
                        let rule = rules.get(&rule).unwrap();
                        if let Some(s) = rule.is_matched(msg, rules) {
                            msg = s;
                        } else {
                            continue 'a;
                        }
                    }
                    ans = Some(msg);
                    break;
                }
                ans
            }
        }
    }
    pub fn is_matched_all(&self, msg: &str, rules: &FxHashMap<usize, Rule>) -> bool {
        if let Some(s) = self.is_matched(msg, rules) {
            s.is_empty()
        } else {
            false
        }
    }
}

fn parse_rule_line(input: &str) -> IResult<&str, (usize, Rule)> {
    separated_pair(complete::usize, tag(": "), parse_rule).parse(input)
}
fn parse_char(input: &str) -> IResult<&str, Rule> {
    delimited(tag("\""), anychar, tag("\""))
        .map(|x| Rule::Char(x))
        .parse(input)
}

fn parse_or(input: &str) -> IResult<&str, Rule> {
    separated_list1(tag(" | "), separated_list1(tag(" "), complete::usize))
        .map(|v| Rule::Or(v))
        .parse(input)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    alt((parse_char, parse_or)).parse(input)
}
fn parse_rules(input: &str) -> IResult<&str, FxHashMap<usize, Rule>> {
    separated_list1(line_ending, parse_rule_line)
        .map(|v: Vec<(usize, Rule)>| {
            v.into_iter().fold(FxHashMap::default(), |mut acc, (i, r)| {
                acc.insert(i, r);
                acc
            })
        })
        .parse(input)
}

fn parse_msg(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        (line_ending, line_ending),
        separated_list1(line_ending, alpha1),
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> (FxHashMap<usize, Rule>, Vec<&str>) {
    all_consuming((parse_rules, parse_msg))
        .parse(input)
        .unwrap()
        .1
}
