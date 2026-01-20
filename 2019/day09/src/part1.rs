use crate::Computer;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut ans = vec![];
    let mut comp = Computer::new(input);
    comp.append_input(1);
    while let Some(output) = comp.long_run() {
        ans.push(output);
    }
    ans.into_iter().next().unwrap() as usize
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_prog_output_itself() {
        let input = parse_input("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let mut comp = Computer::new(input.clone());
        let mut ans = vec![];
        while let Some(output) = comp.long_run() {
            ans.push(output);
        }
        assert_eq!(ans, input);
    }

    #[test]
    fn test_prog_output_16_digits() {
        let input = parse_input("1102,34915192,34915192,7,4,7,99,0");
        let mut comp = Computer::new(input);
        let mut ans = vec![];
        while let Some(output) = comp.long_run() {
            ans.push(output);
        }
        let ans = ans.into_iter().next().unwrap();
        assert_eq!(ans.ilog10(), 15);
    }
    #[test]
    fn test_prog_output_large() {
        let input = parse_input("104,1125899906842624,99");
        let mut comp = Computer::new(input);
        let mut ans = vec![];
        while let Some(output) = comp.long_run() {
            ans.push(output);
        }
        let ans = ans.into_iter().next().unwrap();
        assert_eq!(ans, 1125899906842624);
    }
}
