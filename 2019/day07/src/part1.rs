use crate::Computer;
use crate::parsing::parse_input;
use itertools::Itertools;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut ans = 0usize;

    for v in [0, 1, 2, 3, 4].into_iter().permutations(5) {
        let mut inp = 0;
        for phase in v {
            let mut comp = Computer::new(input.clone());
            comp.append_input(phase);
            comp.append_input(inp);
            if let Some(output) = comp.long_run() {
                inp = output;
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
    #[case("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", 43210)]
    #[case(
        "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        54321
    )]
    #[case(
        "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
        65210
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
