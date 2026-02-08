use crate::manhattan;
use crate::parsing::{Item, parse_input};

pub fn process(_input: &str) -> usize {
    let mut sky = parse_input(_input);

    let mut ans = 0;
    while let Some(p) = sky.pop() {
        ans += 1;
        let mut constellation: Vec<Item> = vec![p];

        loop {
            let mut changed = false;
            let mut new_sky: Vec<Item> = vec![];
            while let Some(b) = sky.pop() {
                if constellation.iter().any(|a| manhattan(*a, b) <= 3) {
                    changed = true;
                    constellation.push(b);
                } else {
                    new_sky.push(b);
                }
            }
            sky = new_sky;
            if !changed {
                break;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#" 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0"#,
        2
    )]
    #[case(
        r#"-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0"#,
        4
    )]
    #[case(
        r#"1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2"#,
        3
    )]
    #[case(
        r#"1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2"#,
        8
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
