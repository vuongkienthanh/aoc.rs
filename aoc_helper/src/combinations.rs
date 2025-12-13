pub fn partition_sum(total: usize, len: usize) -> Vec<Vec<usize>> {
    let mut results = Vec::new();
    let mut current_partition = Vec::with_capacity(len);
    backtrack(total, len, &mut current_partition, &mut results);
    results
}
fn backtrack(
    remaining_total: usize,
    remaining_len: usize,
    current_partition: &mut Vec<usize>,
    results: &mut Vec<Vec<usize>>,
) {
    if remaining_len == 0 {
        if remaining_total == 0 {
            results.push(current_partition.clone());
        }
        return;
    }

    for next_num in 0..=remaining_total {
        current_partition.push(next_num);

        backtrack(
            remaining_total - next_num,
            remaining_len - 1,
            current_partition,
            results,
        );

        current_partition.pop();
    }
}
