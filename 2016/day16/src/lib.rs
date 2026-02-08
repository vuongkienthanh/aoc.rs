pub mod part1;
pub mod part2;

fn dragon_curve(mut a: String, disk: usize) -> String {
    if a.len() >= disk {
        return a[..disk].to_string();
    }
    let b: String = a
        .chars()
        .rev()
        .map(|x| match x {
            '0' => '1',
            '1' => '0',
            _ => panic!("not 0 or 1"),
        })
        .collect();
    a.push('0');
    a.push_str(&b);
    dragon_curve(a, disk)
}

fn checksum(a: String) -> String {
    if a.len() % 2 == 1 {
        return a;
    }
    let b: String = a
        .as_bytes()
        .chunks(2)
        .map(|v| if v[0] == v[1] { '1' } else { '0' })
        .collect();
    checksum(b)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dragon_curve() {
        assert_eq!(
            dragon_curve("10000".to_string(), 20),
            "10000011110010000111".to_string()
        );
    }
    #[test]
    fn test_checksum() {
        assert_eq!(checksum("110010110100".to_string()), "100".to_string());
    }
    #[test]
    fn test_all() {
        let a = "10000".to_string();
        let disk = 20;
        assert_eq!(checksum(dragon_curve(a, disk)), "01100".to_string());
    }
}