pub fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut map = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                map.push((x, y));
            }
        }
    }
    map
}
