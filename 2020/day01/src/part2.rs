use std::cmp::Ordering;
pub fn process(_input: &str) -> usize {
    let mut input: Vec<_> = _input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    input.sort_unstable();
    let mut a = 0;
    let mut b_max = input.len();
    'a: loop {
        let x = input[a];
        b_max = (a + 2..b_max).rfind(|b| input[*b] + x < 2020).unwrap();
        for b in (a + 2..=b_max).rev() {
            let y = input[b];
            for c in a + 1..b {
                let z = input[c];
                match (x + y + z).cmp(&2020) {
                    Ordering::Less => (),
                    Ordering::Greater => break,
                    Ordering::Equal => {
                        break 'a x * y * z;
                    }
                }
            }
        }
        a += 1;
    }
}
