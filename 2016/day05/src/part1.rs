pub fn process(_input: &str) -> String {
    let mut ans = String::new();
    for i in 0..usize::MAX {
        let input = format!("{_input}{i}");
        let digest = md5::compute(input.as_bytes());
        let hex = format!("{:x}", digest);
        if hex.starts_with("00000") {
            ans.push(hex.chars().nth(5).unwrap());
        }
        if ans.len() == 8 {
            break;
        }
    }

    ans
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process("abc"), "18f47a30".to_string());
    }
}
