pub fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

pub fn lcm(a: isize, b: isize) -> isize {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
    .abs()
}

pub fn gcd_vec(nums: &[isize]) -> isize {
    nums.iter().copied().reduce(gcd).unwrap()
}

pub fn lcm_vec(nums: &[isize]) -> isize {
    nums.iter().copied().reduce(lcm).unwrap()
}

pub mod linear_equation_system {
    use fxhash::FxHashSet as Set;
    use itertools::Itertools;

    pub type Matrix = Vec<(Vec<usize>, usize)>;
    pub fn create_matrix(buttons: Vec<Vec<usize>>, target: Vec<usize>) -> Matrix {
        let mut ans: Vec<(Vec<usize>, usize)> = target.into_iter().map(|t| (vec![], t)).collect();

        for (i, button) in buttons.into_iter().enumerate() {
            for sw in button {
                ans[sw].0.push(i);
            }
        }

        ans
    }

    pub fn expand(matrix: &mut Matrix, btn_len: usize) {
        let mut new_matrix: Set<(Vec<usize>, usize)> = matrix.iter().cloned().collect();
        let mut expanded = false;
        for comb in matrix.iter().combinations(3) {
            let actual_rows = [comb[0], comb[1], comb[2]];
            'case: for case in [[0, 1, 2], [0, 2, 1], [1, 2, 0]] {
                let result_target = match (actual_rows[case[0]].1 + actual_rows[case[1]].1)
                    .checked_sub(actual_rows[case[2]].1)
                {
                    Some(x) => x,
                    None => continue 'case,
                };
                let mut result_row = vec![0isize; btn_len];
                for col in actual_rows[case[0]].0.as_slice() {
                    result_row[*col] += 1;
                }
                for col in actual_rows[case[1]].0.as_slice() {
                    result_row[*col] += 1;
                }
                for col in actual_rows[case[2]].0.as_slice() {
                    result_row[*col] -= 1;
                }
                for col in 0..btn_len {
                    if result_row[col] < 0 || result_row[col] > 1 {
                        continue 'case;
                    }
                }
                let result_row: Vec<usize> = result_row
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, x)| if x > 0 { Some(i) } else { None })
                    .collect();
                if result_row.is_empty() {
                    continue 'case;
                }
                if new_matrix.insert((result_row, result_target)) {
                    expanded = true;
                }
            }
        }
        *matrix = new_matrix.into_iter().collect();
        if expanded {
            expand(matrix, btn_len);
        }
    }

    pub fn sort(matrix: &mut Matrix) {
        *matrix = Set::from_iter(matrix.iter().cloned()).into_iter().collect();
        matrix.sort_unstable_by(|a, b| b.0.len().cmp(&a.0.len()));
    }

    pub fn minify(matrix: &mut Matrix) -> bool {
        if matrix.len() <= 1 {
            return true;
        }
        sort(matrix);
        let mut minified = false;
        'top: for i in 0..matrix.len() - 1 {
            for j in i + 1..matrix.len() {
                let j_row = matrix[j].0.clone();
                if j_row.iter().all(|b| matrix[i].0.contains(b)) {
                    match matrix[i].1.checked_sub(matrix[j].1) {
                        Some(x) => {
                            matrix[i].1 = x;
                            minified = true;
                            matrix[i].0.retain(|a| !j_row.contains(a));
                            continue 'top;
                        }
                        None => return false,
                    }
                }
            }
        }

        if minified {
            minify(matrix);
            sort(matrix);
        }
        true
    }
}
