pub fn process(_input: &str) -> usize {
    ('a'..='z').map(|c| react(_input, c).len()).min().unwrap()
}
fn react(input: &str, rm: char) -> Vec<u8> {
    let mut left: Vec<u8> = Vec::new();
    let mut right: Vec<u8> = Vec::new();
    right.extend(input.bytes().rev());
    let (rm0, rm1) = (rm as u8, rm as u8 - 32);

    while let Some(r) = right.pop() {
        if r == rm0 || r == rm1 {
            continue;
        }
        if let Some(l) = left.pop() {
            if l.abs_diff(r) != 32 {
                left.push(l);
                left.push(r);
            }
        } else {
            left.push(r);
        }
    }

    left
}
