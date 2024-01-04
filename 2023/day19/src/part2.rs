use crate::{CompareSign, MachinePartAttribute, PredicationResult};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Predicate {
    attr: MachinePartAttribute,
    sign: CompareSign,
    value: usize,
}

#[derive(Debug)]
struct Workflow {
    predication: Option<Predicate>,
    result: PredicationResult,
}

fn parse_input(input: &str) -> HashMap<String, Vec<Workflow>> {
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
                        let predication = Some(Predicate { attr, sign, value });
                        let result = PredicationResult::new(result);
                        Workflow {
                            predication,
                            result,
                        }
                    } else {
                        let result = PredicationResult::new(p);
                        Workflow {
                            predication: None,
                            result,
                        }
                    }
                })
                .collect::<Vec<_>>();

            (name, workflows)
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}
impl Range {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone)]
struct State {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
    dst: Option<PredicationResult>,
}

impl State {
    #[rustfmt::skip]
    fn replace_x(mut self, x:Range) -> Self { self.x = x; self }
    #[rustfmt::skip]
    fn replace_m(mut self, m:Range) -> Self { self.m = m; self }
    #[rustfmt::skip]
    fn replace_a(mut self, a:Range) -> Self { self.a = a; self }
    #[rustfmt::skip]
    fn replace_s(mut self, s:Range) -> Self { self.s = s; self }
    #[rustfmt::skip]
    fn replace_dst(mut self, dst:Option<PredicationResult>) -> Self { self.dst = dst; self }

    fn go_through_once(self, workflows: &[Workflow]) -> Vec<State> {
        let mut ret = vec![self];
        for wf in workflows {
            if let Some(preication) = wf.predication.clone() {
                todo!()
            } else {
                todo!()
            }
        }
        ret
    }

    // Return (Accept, Reject)
    fn divide_by_workflow(&self, workflow: &Workflow) -> (Option<State>, Option<State>) {
        let predicationresult = workflow.result;

        if let Some(predication) = workflow.predication.clone() {
            match predication.attr {
                MachinePartAttribute::X => {
                    let range = self.x.clone();
                    match predication.sign {
                        CompareSign::G => {
                            if predication.value < range.start {
                                return (Some(self.replace_dst(Some(predicationresult))), None);
                            } else if predication.value == range.start {
                                vec![Range::new(range.start + 1, range.end)]
                            } else if predication.value < range.end {
                                vec![
                                    Range::new(range.start, predication.value),
                                    Range::new(predication.value + 1, range.end),
                                ]
                            } else {
                                vec![]
                            }
                        }
                        CompareSign::L => todo!(),
                    }
                }
                MachinePartAttribute::M => self.m.clone(),
                MachinePartAttribute::A => self.a.clone(),
                MachinePartAttribute::S => self.s.clone(),
            };
            let new_range = match predication.sign {
                CompareSign::G => {}
                CompareSign::L => {
                    if predication.value <= range.start {
                        vec![]
                    } else if predication.value < range.end {
                        vec![
                            Range::new(range.start, predication.value - 1),
                            Range::new(predication.value, range.end),
                        ]
                    } else if predication.value == range.end {
                        vec![Range::new(range.start, range.end - 1)]
                    } else {
                        vec![]
                    }
                }
            };
        } else {
            ret.push(State {
                x: self.x.clone(),
                m: self.m.clone(),
                a: self.a.clone(),
                s: self.s.clone(),
                dst: Some(workflow.result.clone()),
            })
        }
        ret
    }
}

pub fn process(input: &str) -> usize {
    let system = parse_input(input);
    let start = State {
        x: Range::new(1, 4000),
        m: Range::new(1, 4000),
        a: Range::new(1, 4000),
        s: Range::new(1, 4000),
        dst: Some(PredicationResult::Refer("in".to_string())),
    };
    0
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
