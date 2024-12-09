use super::{checksum, parse, Block};
pub fn process(_input: &str) -> usize {
    let mut input = parse(_input);

    let mut ans = vec![];
    let mut i = 0;
    while i < input.len() {
        match input.get(i).cloned().unwrap() {
            Block::Free { count: cfree } => match input.pop().unwrap() {
                Block::File { id, count: cfile } => match cfree.cmp(&cfile) {
                    std::cmp::Ordering::Less => {
                        ans.push(Block::File { id, count: cfree });
                        input.push(Block::File {
                            id,
                            count: cfile - cfree,
                        });
                        i += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        ans.push(Block::File { id, count: cfile });
                        i += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        ans.push(Block::File { id, count: cfile });
                        *input.get_mut(i).unwrap() = Block::Free {
                            count: cfree - cfile,
                        };
                    }
                },
                _ => continue,
            },
            file => {
                ans.push(file.clone());
                i += 1;
            }
        }
    }
    checksum(&ans)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"2333133121414131402"#;
        assert_eq!(process(input), 1928);
    }
}
