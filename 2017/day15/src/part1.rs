const MASK: usize = 0b1_111_111_111_111_111;

pub fn process(_input: &str) -> usize {
    let mut input = _input
        .lines()
        .map(|line| line.split_ascii_whitespace().last().unwrap())
        .map(|x| x.parse::<usize>().unwrap());
    let a = input.next().unwrap();
    let b = input.next().unwrap();

    run(a, b)
}

fn run(mut a: usize, mut b: usize) -> usize {
    (0..40_000_000)
        .filter(|_| {
            a = a * 16807 % 2147483647;
            b = b * 48271 % 2147483647;
            a & MASK == b & MASK
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(run(65, 8921), 588);
    }
}
