pub fn process(_input: &str) -> usize {
    spin(_input.parse::<usize>().unwrap())
}

fn spin(step: usize) -> usize {
    // (prev, next)
    let mut map = [(0, 0); 2018];
    let mut len = 1;
    let mut current_node = 0;
    for i in 1..=2017 {
        let remain_step = step % len;
        for _ in 0..remain_step {
            current_node = map[current_node].1;
        }
        let next_node = map[current_node].1;

        // insert new node
        map[current_node].1 = i;
        map[next_node].0 = i;
        map[i] = (current_node, next_node);

        current_node = i;
        len += 1;
    }
    map[2017].1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(spin(3), 638);
    }
}
