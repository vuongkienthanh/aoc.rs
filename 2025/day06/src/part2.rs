use crate::parsing::Item;

pub fn process(_input: &str) -> usize {
    let len = _input.lines().count();
    let ops: Vec<Item> = _input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| match x {
            "*" => Item::Mul,
            "+" => Item::Add,
            _ => panic!(),
        })
        .collect();

    let mut iter_col = _input
        .lines()
        .take(len - 1)
        .map(|line| line.chars())
        .collect::<Vec<_>>();

    let mut ans = 0;

    for op in ops {
        let mut v = vec![];
        let mut space = 0;
        'a: loop {
            let mut num = 0;
            for i in &mut iter_col {
                if let Some(c) = i.next() {
                    if c == ' ' {
                        space += 1;
                    } else {
                        num = num * 10 + c.to_digit(10).unwrap() as usize;
                    }
                } else {
                    break 'a;
                }
            }
            if space == iter_col.len() {
                break;
            } else {
                space = 0;
                v.push(num);
            }
        }
        ans += match op {
            Item::Add => v.into_iter().sum::<usize>(),
            Item::Mul => v.into_iter().product::<usize>(),
        }
    }
    ans
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 3263827);
    }
}
