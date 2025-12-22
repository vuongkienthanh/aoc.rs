pub mod parsing;
pub mod part1;
pub mod part2;

type Quantities = Vec<usize>;
const MAX: usize = 100;

fn build_all_combinations(max_len: usize) -> Vec<Quantities> {
    fn build(max_len: usize, cur_len: usize, ret: &mut Vec<Quantities>) {
        if cur_len == max_len - 1 {
            for comb in ret.iter_mut() {
                let comb_sum = comb.iter().sum::<usize>();
                let remain = MAX - comb_sum;
                comb.push(remain);
            }
            return;
        }

        let mut new_ret = vec![];

        while let Some(comb) = ret.pop() {
            let comb_sum = comb.iter().sum::<usize>();
            let max_remain = MAX - comb_sum;
            for remain in 0..=max_remain {
                let mut next_comb = comb.clone();
                next_comb.push(remain);
                new_ret.push(next_comb);
            }
        }

        ret.extend(new_ret);
        build(max_len, cur_len + 1, ret);
    }

    let mut ret = vec![vec![]];
    build(max_len, 0, &mut ret);

    ret
}

fn score(quantities: &Quantities, ingredients: &[[isize; 5]]) -> usize {
    assert_eq!(quantities.len(), ingredients.len());
    let property_scores = quantities
        .iter()
        .zip(ingredients.iter())
        .fold([0; 4], |acc, (q, i)| {
            [
                acc[0] + *q as isize * i[0],
                acc[1] + *q as isize * i[1],
                acc[2] + *q as isize * i[2],
                acc[3] + *q as isize * i[3],
            ]
        });
    if property_scores.iter().any(|x| x <= &0) {
        0
    } else {
        property_scores.iter().product::<isize>() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::parsing::parse_input;
    use super::*;
    use rstest::*;
    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#
    }
    #[rstest]
    fn test_score(fixture: &str) {
        let input = parse_input(fixture);
        let quantities = vec![44, 56];
        assert_eq!(score(&quantities, &input), 62842880);
    }
}
