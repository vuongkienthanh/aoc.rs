pub fn process(_input: &str) -> String {
    let input: Vec<&str> = _input.lines().collect();
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            if let Some(idx) = diff_exact_once(input[i], input[j]) {
                return input[i]
                    .chars()
                    .take(idx)
                    .chain(input[i].chars().skip(idx + 1))
                    .collect();
            }
        }
    }
    panic!("should have an answer")
}

fn diff_exact_once(a: &str, b: &str) -> Option<usize> {
    let mut diff_one = false;
    let mut ans = None;
    for ((i, x), y) in a.char_indices().zip(b.chars()) {
        if x != y {
            if !diff_one {
                diff_one = true;
                ans = Some(i);
            } else {
                return None;
            }
        }
    }
    ans
}
