use md5::{Digest, Md5};

pub fn process(_input: &str) -> usize {
    (0..usize::MAX)
        .find(|i| {
            let d = Md5::digest(format!("{_input}{i}"));
            d[0..3] == [0, 0, 0]
        })
        .unwrap()
}