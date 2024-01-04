use crate::{CompareSign, MachinePartAttribute, PredicationResult};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}
impl Range {
    fn try_new(start: usize, end: usize) -> Option<Self> {
        if end >= start {
            Some(Self { start, end })
        } else {
            None
        }
    }
    fn len(&self) -> usize {
        self.end - self.start + 1
    }
}

#[derive(Debug, Clone)]
struct RangeXMAS {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}
impl RangeXMAS {
    #[rustfmt::skip]
    fn replace_x(mut self, x:Range) -> Self { self.x = x; self }
    #[rustfmt::skip]
    fn replace_m(mut self, m:Range) -> Self { self.m = m; self }
    #[rustfmt::skip]
    fn replace_a(mut self, a:Range) -> Self { self.a = a; self }
    #[rustfmt::skip]
    fn replace_s(mut self, s:Range) -> Self { self.s = s; self }
    fn combinations(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

#[derive(Debug, Clone)]
struct Condition {
    attr: MachinePartAttribute,
    sign: CompareSign,
    value: usize,
}

impl Condition {
    // return ( accept, reject )
    fn eval(&self, rangexmas: &RangeXMAS) -> (Option<RangeXMAS>, Option<RangeXMAS>) {
        let range = match self.attr {
            MachinePartAttribute::X => &rangexmas.x,
            MachinePartAttribute::M => &rangexmas.m,
            MachinePartAttribute::A => &rangexmas.a,
            MachinePartAttribute::S => &rangexmas.s,
        };
        let mut accept = None;
        let mut reject = None;
        match self.sign {
            CompareSign::G => {
                if self.value < range.start {
                    accept = Range::try_new(range.start, range.end);
                } else if self.value < range.end {
                    accept = Range::try_new(self.value + 1, range.end);
                    reject = Range::try_new(range.start, self.value);
                } else {
                    reject = Range::try_new(range.start, range.end);
                }
            }
            CompareSign::L => {
                if self.value > range.end {
                    accept = Range::try_new(range.start, range.end);
                } else if self.value > range.start {
                    accept = Range::try_new(range.start, self.value - 1);
                    reject = Range::try_new(self.value, range.end);
                } else {
                    reject = Range::try_new(range.start, range.end);
                }
            }
        }
        match self.attr {
            MachinePartAttribute::X => (
                accept.map(|range| rangexmas.clone().replace_x(range)),
                reject.map(|range| rangexmas.clone().replace_x(range)),
            ),
            MachinePartAttribute::M => (
                accept.map(|range| rangexmas.clone().replace_m(range)),
                reject.map(|range| rangexmas.clone().replace_m(range)),
            ),
            MachinePartAttribute::A => (
                accept.map(|range| rangexmas.clone().replace_a(range)),
                reject.map(|range| rangexmas.clone().replace_a(range)),
            ),
            MachinePartAttribute::S => (
                accept.map(|range| rangexmas.clone().replace_s(range)),
                reject.map(|range| rangexmas.clone().replace_s(range)),
            ),
        }
    }
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    result: PredicationResult,
}
type Workflow = Vec<Rule>;

fn parse_input(input: &str) -> HashMap<String, Workflow> {
    input
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(|line| {
            let name = line.chars().take_while(|c| *c != '{').collect::<String>();
            let workflows = line
                .chars()
                .skip_while(|c| *c != '{')
                .skip(1)
                .take_while(|c| *c != '}')
                .collect::<String>()
                .split(',')
                .map(|p| {
                    if p.contains(':') {
                        let attr = MachinePartAttribute::new(p.chars().take(1).next().unwrap());
                        let sign = CompareSign::new(p.chars().skip(1).take(1).next().unwrap());
                        let (compare_value, result) = &p[2..].split_once(':').unwrap();
                        let value = compare_value.parse::<usize>().unwrap();
                        let condition = Some(Condition { attr, sign, value });
                        let result = PredicationResult::new(result);
                        Rule { condition, result }
                    } else {
                        let result = PredicationResult::new(p);
                        let condition = None;
                        Rule { condition, result }
                    }
                })
                .collect::<Vec<_>>();
            (name, workflows)
        })
        .collect()
}
//
#[derive(Debug)]
struct State {
    rangexmas: RangeXMAS,
    result: PredicationResult,
}

impl State {
    fn go_into_workflow(&self, system: &HashMap<String, Workflow>) -> Vec<State> {
        if let PredicationResult::Refer(name) = &self.result {
            let workflow = system.get(name).unwrap();
            let mut ret = vec![];
            let mut range = self.rangexmas.clone();

            for rule in workflow {
                if let Some(condition) = &rule.condition {
                    let (accept, reject) = condition.eval(&range);
                    if let Some(accept_range) = accept {
                        ret.push(State {
                            rangexmas: accept_range,
                            result: rule.result.clone(),
                        })
                    };
                    if let Some(reject_range) = reject {
                        range = reject_range
                    };
                } else {
                    ret.push(State {
                        rangexmas: range.clone(),
                        result: rule.result.clone(),
                    })
                }
            }

            ret
        } else {
            panic!("should not call go_into_workflow on an accepted ")
        }
    }
}

pub fn process(input: &str) -> usize {
    let system = parse_input(input);
    let start = State {
        rangexmas: RangeXMAS {
            x: Range::try_new(1, 4000).unwrap(),
            m: Range::try_new(1, 4000).unwrap(),
            a: Range::try_new(1, 4000).unwrap(),
            s: Range::try_new(1, 4000).unwrap(),
        },
        result: PredicationResult::Refer("in".to_string()),
    };
    let mut states = std::collections::VecDeque::new();
    states.push_back(start);
    let mut parsed_states: Vec<State> = vec![];

    while let Some(state) = states.pop_front() {
        for new_state in state.go_into_workflow(&system) {
            match new_state.result {
                PredicationResult::Refer(_) => states.push_back(new_state),
                ref _other => parsed_states.push(new_state),
            }
        }
    }

    parsed_states
        .into_iter()
        .filter(|state| state.result == PredicationResult::Accept)
        .map(|state| state.rangexmas.combinations())
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;
        assert_eq!(process(input), 167409079868000);
    }
}
