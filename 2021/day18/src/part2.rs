use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut ans = usize::MIN;

    for (i, a) in input.iter().cloned().enumerate() {
        for (j, b) in input.iter().cloned().enumerate() {
            if i != j {
                ans = ans.max(a.clone().add(b).magnitude());
            }
        }
    }
    ans
}
