use crate::Computer;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut comp = Computer::new_with(input, 12, 2);
    comp.run_to_finish();
    comp.prog[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("1,9,10,3,2,3,11,0,99,30,40,50", "3500,9,10,70,2,3,11,0,99,30,40,50")]
    #[case("1,0,0,0,99", "2,0,0,0,99")]
    #[case("2,3,0,3,99", "2,3,0,6,99")]
    #[case("2,4,4,5,99,0", "2,4,4,5,99,9801")]
    #[case("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99")]
    fn test_run(#[case] input: &str, #[case] expected: &str) {
        let mut comp = Computer::new(parse_input(input));
        comp.run_to_finish();
        assert_eq!(comp.prog, parse_input(expected),);
    }
}
