pub fn process(_input: &str) -> usize {
    spin(_input.parse::<usize>().unwrap(), 50_000_000)
}

fn spin(step: usize, count: usize) -> usize {
    let mut len = 1;
    let mut current_pos = 0;
    let mut zero_next = 0;
    for i in 1..=count {
        current_pos = (current_pos + step) % len;
        if current_pos == 0 {
            zero_next = i;
        }
        current_pos += 1;
        len += 1;
    }
    zero_next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(spin(3, 9), 9);
    }
}
