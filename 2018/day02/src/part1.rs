pub fn process(_input: &str) -> usize {
    let mut two = 0;
    let mut three = 0;
    for line in _input.lines() {
        let mut arr = [0; 26];
        for c in line.bytes() {
            let c = c - b'a';
            arr[c as usize] += 1;
        }
        if arr.contains(&2) {
            two += 1;
        }
        if arr.contains(&3) {
            three += 1;
        }
    }

    two * three
}
