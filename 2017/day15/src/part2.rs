use tinyvec::ArrayVec;

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
    let mut ans = 0;

    for _ in 0..5_000 {
        let (arr_a, arr_b) = find_1000(a, b);
        ans += arr_a
            .into_iter()
            .zip(arr_b)
            .filter(|(a, b)| a & MASK == b & MASK)
            .count();
        a = arr_a.last().cloned().unwrap();
        b = arr_b.last().cloned().unwrap();
    }

    ans
}

fn find_1000(a: usize, b: usize) -> (ArrayVec<[usize; 1000]>, ArrayVec<[usize; 1000]>) {
    let mut zip = [(a, 16807, 4), (b, 48271, 8)]
        .into_iter()
        .map(|(mut x, m, d)| {
            let mut arr = ArrayVec::<[usize; 1000]>::new();
            for _ in 0.. {
                x = x * m % 2147483647;
                if x.is_multiple_of(d) {
                    arr.push(x);
                }
                if arr.len() == 1000 {
                    break;
                }
            }
            arr
        });
    (zip.next().unwrap(), zip.next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(run(65, 8921), 309);
    }
}
