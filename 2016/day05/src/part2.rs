pub fn process(_input: &str) -> String {
    let mut ans = ['*'; 8];

    for i in 0..usize::MAX {
        let input = format!("{_input}{i}");
        let digest = md5::compute(input.as_bytes());
        let hex = format!("{:x}", digest);
        if hex.starts_with("00000") {
            let sixth = hex.chars().nth(5).unwrap();
            if let Some(position) = sixth.to_digit(10)
                && (0..8).contains(&position)
            {
                let p = position as usize;
                if ans[p] == '*' {
                    ans[p] = hex.chars().nth(6).unwrap();
                }
            }
        }
        if ans.iter().all(|c| *c != '*') {
            break;
        }
    }

    ans.into_iter().collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process("abc"), "05ace8e3".to_string());
    }
}
