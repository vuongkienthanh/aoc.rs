use crate::parsing::parse_input;
use crate::*;
use aoc_helper::combinations::partition_sum;
use fxhash::FxHashSet as Set;
use rayon::prelude::*;
use std::collections::VecDeque;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input
        // .into_par_iter()
        .into_iter()
        .enumerate()
        // .skip(3)
        // .take(1)
        .map(move |(tc, (_, buttons, target))| {
            let btn_len = buttons.len();
            // let target_len = target.len();
            let result: Vec<Option<usize>> = vec![None; btn_len];
            let mut matrix = create_matrix(buttons, target);
            // println!("initial matrix: {matrix:?}");
            expand(&mut matrix, btn_len);
            minify(&mut matrix);
            // expand(&mut matrix, btn_len);
            // println!("after expanded: {matrix:?}");

            // println!(
            //     "x = {} in {}",
            //     matrix.iter().map(|(x, _)| x.len()).sum::<usize>(),
            //     btn_len * target_len
            // );

            let mut min = usize::MAX;
            let mut queue = VecDeque::new();
            queue.push_back((matrix, result));

            while let Some((mut matrix, result)) = queue.pop_front() {
                println!(
                    "matrix: {matrix:?}  result={result:?} in queue: {}",
                    queue.len()
                );
                // finish this combination
                if result.iter().all(|x| x.is_some()) {
                    min = min.min(result.into_iter().flatten().sum());
                    continue;
                }

                let (last_row, total) = matrix.pop().unwrap();
                let need = last_row.len();
                // println!("last_row = {last_row:?} need={need} total={total}");

                let combinations = partition_sum(total, need);
                // let mut set = Set::default();

                'left: for comb in combinations.iter() {
                    let mut new_matrix = matrix.clone();
                    let mut new_result = result.clone();
                    for (col, value) in last_row.iter().zip(comb) {
                        for (row, t) in new_matrix.iter_mut() {
                            if row.contains(col) {
                                match t.checked_sub(*value) {
                                    Some(new_t) => {
                                        *t = new_t;
                                        row.retain(|x| x != col);
                                    }
                                    None => {
                                        // println!("push failed");
                                        continue 'left;
                                    }
                                }
                            }
                        }
                        new_result[*col] = Some(*value);
                    }
                    if minify(&mut new_matrix) {
                        // println!("push succeed");
                        // set.insert((new_matrix, new_result));
                        queue.push_back((new_matrix, new_result))
                    }
                }
                // 'right: for comb in combinations.iter().rev() {
                //     let mut new_matrix = matrix.clone();
                //     let mut new_result = result.clone();
                //     for (col, value) in last_row.iter().zip(comb) {
                //         for (row, t) in new_matrix.iter_mut() {
                //             if row.contains(col) {
                //                 match t.checked_sub(*value) {
                //                     Some(new_t) => {
                //                         *t = new_t;
                //                         row.retain(|x| x != col);
                //                     }
                //                     None => {
                //                         // println!("push failed");
                //                         break 'right;
                //                     }
                //                 }
                //             }
                //         }
                //         new_result[*col] = Some(*value);
                //     }
                //     if minify(&mut new_matrix) {
                //         // println!("push succeed");
                //         set.insert((new_matrix, new_result));
                //     }
                // }
                // queue.extend(set.into_iter());
            }
            println!("done row {tc} :found min = {min}");
            println!("===============================================");
            min
            // 0
        })
        .sum::<usize>()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 33);
    }
}
