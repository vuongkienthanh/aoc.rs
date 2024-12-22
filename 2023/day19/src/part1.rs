use crate::{MachinePartAttribute, PredicationResult};
use predicates::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Condition {
    attr: MachinePartAttribute,
    predication: predicates::BoxPredicate<usize>,
}
impl Condition {
    fn eval(&self, machinepart: &MachinePart) -> bool {
        match self.attr {
            MachinePartAttribute::X => self.predication.eval(&machinepart.x),
            MachinePartAttribute::M => self.predication.eval(&machinepart.m),
            MachinePartAttribute::A => self.predication.eval(&machinepart.a),
            MachinePartAttribute::S => self.predication.eval(&machinepart.s),
        }
    }
}
#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    result: PredicationResult,
}
type Workflow = Vec<Rule>;

#[derive(Debug)]
struct MachinePart {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
impl MachinePart {
    fn go_through<'a>(&'a self, system: &'a HashMap<String, Workflow>) -> &'a PredicationResult {
        let mut workflow = system.get("in").unwrap();
        loop {
            let mut result = None;
            for rule in workflow {
                if let Some(condition) = &rule.condition {
                    if condition.eval(self) {
                        result = Some(&rule.result);
                        break;
                    }
                } else {
                    result = Some(&rule.result);
                    break;
                }
            }
            match result.unwrap() {
                PredicationResult::Refer(name) => workflow = system.get(name).unwrap(),
                other => return other,
            }
        }
    }
    fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<MachinePart>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let mut system = HashMap::new();
    for line in workflows.lines() {
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
                    let sign = p.chars().skip(1).take(1).next().unwrap();
                    let (compare_value, result) = &p[2..].split_once(':').unwrap();
                    let compare_value = compare_value.parse::<usize>().unwrap();
                    let predication = match sign {
                        '>' => predicate::gt(compare_value).boxed(),
                        '<' => predicate::lt(compare_value).boxed(),
                        _ => panic!("not < or >"),
                    };
                    let result = PredicationResult::new(result);
                    Rule {
                        condition: Some(Condition { attr, predication }),
                        result,
                    }
                } else {
                    let result = PredicationResult::new(p);
                    Rule {
                        condition: None,
                        result,
                    }
                }
            })
            .collect::<Vec<_>>();
        system.insert(name, workflows);
    }

    let machineparts = parts
        .lines()
        .map(|line| {
            let mut split = line[1..line.len() - 1].split(',');
            let x = split.next().unwrap()[2..].parse::<usize>().unwrap();
            let m = split.next().unwrap()[2..].parse::<usize>().unwrap();
            let a = split.next().unwrap()[2..].parse::<usize>().unwrap();
            let s = split.next().unwrap()[2..].parse::<usize>().unwrap();
            MachinePart { x, m, a, s }
        })
        .collect::<Vec<_>>();

    (system, machineparts)
}

pub fn process(input: &str) -> usize {
    let (system, machineparts) = parse_input(input);
    machineparts
        .into_iter()
        .filter(|mp| mp.go_through(&system) == &PredicationResult::Accept)
        .map(|mp| mp.rating())
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
        assert_eq!(process(input), 19114);
    }
}
