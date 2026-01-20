use crate::Computer;
use crate::parsing::parse_input;
use itertools::Itertools;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut ans = 0usize;

    for v in [5, 6, 7, 8, 9].into_iter().permutations(5) {
        let mut comps: Vec<Computer> = v
            .into_iter()
            .map(|phase| {
                let mut comp = Computer::new(input.clone());
                comp.append_input(phase);
                comp
            })
            .collect();
        let mut inp = 0;
        for i in (0..5).into_iter().cycle() {
            comps[i].append_input(inp);
            match comps[i].long_run() {
                Some(output) => {
                    inp = output;
                }
                None => break,
            }
        }
        ans = ans.max(inp as usize);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        139629729
    )]
    #[case(
        "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        18216
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
