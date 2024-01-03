use crate::{CompareSign, MachinePartAttribute, PredicationResult};
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug)]
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

struct State {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
    dst: PredicationResult,
}

impl State {
    fn go_through(&self, workflows: &[Workflow]) -> Vec<State> {
        let mut ret = vec![];
        for wf in workflows {

        }
        ret
    }
}

pub fn process(input: &str) -> usize {
    let system = parse_input(input);
    dbg!(system);
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
