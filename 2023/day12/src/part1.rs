pub fn process(_input: &str) -> usize {
    todo!();
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::HotSprings;
    use crate::Validness;
    use rstest::rstest;

    #[test]
    fn test_process() {
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
        assert_eq!(process(input), 21);
    }

    #[rstest]
    #[case("???.### 1,1,3", HotSprings { springs: vec!['?','?','?','.','#','#','#'], records: vec![1,1,3] })]
    #[case(".??..??...?##. 1,1,3", HotSprings { springs: vec!['.','?','?','.','?','?','.','?','#','#','.'], records: vec![1,1,3] })]
    #[case("????.######..#####. 1,6,5", HotSprings { springs: vec!['?','?','?','?','.','#','#','#','#','#','#','.','#','#','#','#','#','.'], records: vec![1,6,5] })]
    fn test_hot_springs_new(#[case] input: &str, #[case] expect: HotSprings) {
        assert_eq!(HotSprings::new(input,1), expect)
    }

    #[rstest]
    #[case("???.### 1,1,3", [vec!['.','?','?','.','#','#','#'],vec!['#','?','?','.','#','#','#']])]
    #[case(".??..??...?##. 1,1,3",[vec!['.','.','?','.','?','?','.','?','#','#','.'],vec!['.','#','?','.','?','?','.','?','#','#','.']])]
    #[case("????.######..#####. 1,6,5",[vec!['.','?','?','?','.','#','#','#','#','#','#','.','#','#','#','#','#','.'], vec!['#','?','?','?','.','#','#','#','#','#','#','.','#','#','#','#','#','.']])]
    fn test_hot_springs_replace(#[case] input: &str, #[case] expect: [Vec<char>; 2]) {
        assert_eq!(
            HotSprings::new(input,1).replace_once().map(|s| s.springs),
            expect
        )
    }

    #[rstest]
    #[case("#??.?.### 2,1,3", Validness::LessAndMayValid { last_idx: 0, count: 0 })]
    #[case("#.#??.?.### 1,2,1,3", Validness::LessAndMayValid { last_idx: 1, count: 1 })]
    #[case(".??.### 1,1,3", Validness::EqualAndMayValid { last_idx: 0, count: 0 })]
    #[case("#??.### 1,1,3", Validness::EqualAndMayValid { last_idx: 0, count: 0 } )]
    #[case(".?.### 1,1,3", Validness::EqualAndMayValid { last_idx: 0, count: 0 })]
    #[case(".#?.### 1,1,3", Validness::EqualAndMayValid { last_idx: 0, count: 0 })]
    #[case("#.?.### 1,1,3", Validness::EqualAndMayValid { last_idx: 1, count: 1 } )]
    #[case("##?.### 1,1,3", Validness::NotValid)]
    #[case(".### 1,1,3", Validness::NotValid)]
    #[case(".#.### 1,1,3", Validness::NotValid)]
    #[case(".#.### 1,1,3", Validness::NotValid)]
    #[case(".##.### 1,1,3", Validness::NotValid)]
    #[case("#.### 1,1,3", Validness::NotValid)]
    #[case("#.#.### 1,1,3", Validness::EqualAndMayValid { last_idx: 3, count: 2 } )]
    fn test_hot_springs_valid(#[case] input: &str, #[case] expect: Validness) {
        assert_eq!(HotSprings::new(input,1).is_valid(), expect,)
    }

    #[rstest]
    #[case("#.#??.?.### 1,2,1,3", 1,1, vec!['#','?','?','.','?','.','#','#','#'])]
    #[case("#.?.### 1,1,3", 1,1, vec!['?','.','#','#','#'])]
    #[case("#.#.### 1,1,3",3,2, vec!['#','#','#'])]
    fn test_hot_springs_trim(
        #[case] input: &str,
        #[case] last_idx: usize,
        #[case] count: usize,
        #[case] expect: Vec<char>,
    ) {
        let mut hs = HotSprings::new(input,1);
        hs.trim_when_valid(last_idx, count);
        assert_eq!(hs.springs, expect)
    }
}
