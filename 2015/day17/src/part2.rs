pub fn process(_input: &str) -> usize {
    let input = _input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    count_all_combinations(input, 150)
}

fn count_all_combinations(containers: Vec<usize>, liters: usize) -> usize {
    fn build(
        containers: &[usize],
        liters: usize,
        mut v: Vec<Vec<usize>>,
        mut min: usize,
        ans: &mut usize,
    ) {
        use std::cmp::Ordering::*;
        let mut new_v = vec![];
        while let Some(used_indices) = v.pop() {
            let current_volume: usize = used_indices.iter().map(|i| containers[*i]).sum();
            let missing_volume = liters - current_volume;
            let last_index = used_indices.last().copied().unwrap();

            for (i, item) in containers.iter().enumerate().skip(last_index + 1) {
                match item.cmp(&missing_volume) {
                    Greater => (),
                    Equal => {
                        let current_len = used_indices.len() + 1;
                        match current_len.cmp(&min) {
                            Greater => (),
                            Equal => *ans += 1,
                            Less => {
                                min = current_len;
                                *ans = 1;
                            }
                        }
                    }
                    Less => {
                        let mut new_used_indices = used_indices.clone();
                        new_used_indices.push(i);
                        new_v.push(new_used_indices);
                    }
                }
            }
        }

        if !new_v.is_empty() {
            build(containers, liters, new_v, min, ans)
        }
    }

    let v = (0..containers.len()).map(|i| vec![i]).collect();
    let min = usize::MAX;
    let mut ans = 0;

    build(&containers, liters, v, min, &mut ans);
    ans
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        assert_eq!(count_all_combinations(vec![20, 15, 10, 5, 5], 25), 3);
    }
}
