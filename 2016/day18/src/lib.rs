pub mod part1;
pub mod part2;

fn count_safe_tiles(input: &str, rows: usize) -> usize {
    let len = input.len();
    let mut row: u128 = 0;
    // safe tiles is 1, trap is 0
    for c in input.chars() {
        row <<= 1;
        if c == '.' {
            row |= 1;
        }
    }
    let mut ans = row.count_ones();
    let mut last_row = row;
    for _ in 0..rows - 1 {
        let imaginary_row = (((1u128 << len) | last_row) << 1) | 1;
        let mut new_row = 0u128;
        for i in 1..len + 1 {
            new_row <<= 1;
            let right = (imaginary_row & (1 << (i - 1))) >> (i - 1);
            let center = (imaginary_row & (1 << i)) >> i;
            let left = (imaginary_row & (1 << (i + 1))) >> (i + 1);
            // if not is_trap
            if !((left == 0 && center == 0 && right == 1)
                || (left == 1 && center == 0 && right == 0)
                || (left == 0 && center == 1 && right == 1)
                || (left == 1 && center == 1 && right == 0))
            {
                new_row |= 1;
            }
        }
        ans += new_row.count_ones();
        last_row = new_row;
    }
    ans as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_safe_tiles() {
        assert_eq!(count_safe_tiles(".^^.^.^^^^", 10), 38);
    }
}
