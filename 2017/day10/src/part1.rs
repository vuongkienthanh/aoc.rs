use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut list: Vec<usize> = (0..256usize).into_iter().collect();
    transform(&mut list, input);
    list.into_iter().take(2).product()
}

fn transform(list: &mut [usize], input: Vec<usize>) {
    let mut i = 0;
    let mut skip_size = 0;

    for length in input {
        let half = length / 2;
        for left in 0..half {
            let right = length - 1 - left;
            list.swap((i + left) % list.len(), (i + right) % list.len());
        }
        i = (i + length + skip_size) % list.len();
        skip_size += 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let mut list = vec![0, 1, 2, 3, 4];
        transform(&mut list, vec![3, 4, 1, 5]);
        assert_eq!(list.into_iter().take(2).product::<usize>(), 12);
    }
}
