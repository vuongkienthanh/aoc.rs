fn decimal(input: &str) -> isize {
    input
        .chars()
        .rev()
        .map(|c| match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        })
        .enumerate()
        .map(|(i, c)| c * 5isize.pow(i as u32))
        .sum()
}

fn SNAFU(mut input: isize) -> String {
    let mut ans = String::new();
    while input > 0 {
        match input % 5 {
            0 => ans.push('0'),
            1 => {
                ans.push('1');
                input -= 1;
            }
            2 => {
                ans.push('2');
                input -= 2;
            }
            3 => {
                ans.push('=');
                input += 2;
            }
            4 => {
                ans.push('-');
                input += 1;
            }
            _ => (),
        }
        input /= 5;
    }
    ans.chars().rev().collect()
}

pub fn process(_input: &str) -> String {
    let s = _input.lines().map(|line| decimal(line)).sum();
    SNAFU(s)
}