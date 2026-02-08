use crate::Computer;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    for a in 0..100 {
        for b in 0..100 {
            let mut comp = Computer::new_with(input.clone(), a, b);
            comp.run_to_finish();
            if comp.prog[0] == 19690720 {
                return 100 * a + b;
            }
        }
    }
    panic!("should have an answer")
}
