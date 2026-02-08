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
        for i in 0..len {
            new_row <<= 1;
            let triplet = (imaginary_row & (0b111 << i)) >> i;
            // if not is_trap
            if !((triplet == 0b001)
                || (triplet == 0b100)
                || (triplet == 0b011)
                || (triplet == 0b110))
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
