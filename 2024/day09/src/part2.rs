use super::{checksum, parse, Block};
pub fn process(_input: &str) -> usize {
    let mut input = parse(_input);

    let mut ans = vec![];
    let mut i = 0;
    while i < input.len() {
        match input.get(i).cloned().unwrap() {
            Block::Free { count: cfree } => match input.pop().unwrap() {
                Block::File { id, count: cfile } => match cfree.cmp(&cfile) {
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
                    std::cmp::Ordering::Less => {
                        if let Some((j, id2, cfile2)) = (i + 1..input.len())
                            .rev()
                            .filter_map(|j| match input.get(j) {
                                Some(Block::File { id, count }) => Some((j, *id, *count)),
                                _ => None,
                            })
                            .find(|(_, _, cfile2)| cfree >= *cfile2)
                        {
                            match cfree.cmp(&cfile2) {
                                std::cmp::Ordering::Greater => {
                                    ans.push(Block::File {
                                        id: id2,
                                        count: cfile2,
                                    });
                                    *input.get_mut(j).unwrap() = Block::Free { count: cfile2 };
                                    *input.get_mut(i).unwrap() = Block::Free {
                                        count: cfree - cfile2,
                                    };
                                }
                                std::cmp::Ordering::Equal => {
                                    ans.push(Block::File {
                                        id: id2,
                                        count: cfree,
                                    });
                                    *input.get_mut(j).unwrap() = Block::Free { count: cfile2 };
                                    i += 1;
                                }
                                _ => (),
                            }
                        } else {
                            i += 1;
                            ans.push(Block::Free { count: cfree });
                        }
                        input.push(Block::File { id, count: cfile });
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
        assert_eq!(process(input), 2858);
    }
}
