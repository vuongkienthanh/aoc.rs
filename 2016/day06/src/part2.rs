use micromap::Map;

pub fn process(_input: &str) -> String {
    _input
        .lines()
        .fold(Vec::<Map<char, usize, 26>>::new(), |mut acc, line| {
            for (i, c) in line.char_indices() {
                if let Some(m) = acc.get_mut(i) {
                    *m.entry(c).or_default() += 1;
                } else {
                    let mut m = Map::new();
                    m.insert(c, 1);
                    acc.push(m);
                }
            }
            acc
        })
        .into_iter()
        .map(|m| {
            m.into_iter()
                .min_by_key(|(_, x)| *x)
                .map(|(c, _)| c)
                .unwrap()
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"#
    }
    #[rstest]
    fn test_process_1(fixture: &str) {
        assert_eq!(process(fixture), "advent");
    }
}
