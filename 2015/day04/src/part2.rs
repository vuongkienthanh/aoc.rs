pub fn process(_input: &str) -> usize {
    for i in 0..usize::MAX {
        let inp = format!("{}{}", _input, i);
        let hex = format!("{:x}", md5::compute(inp.as_bytes()));
        if hex.starts_with("000000") {
            return i;
        }
    }
    panic!("should have an answer")
}
