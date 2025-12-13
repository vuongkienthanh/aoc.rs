use super::parse;
use glam::IVec2;
use std::collections::HashSet;

pub fn process(_input: &str) -> usize {
    let (rows, cols, map) = parse(_input);
    let mut antinodes = HashSet::<IVec2>::new();

    for v in map.into_values() {
        for i in 0..v.len() - 1 {
            for j in i + 1..v.len() {
                let a = v.get(i).unwrap();
                let b = v.get(j).unwrap();
                let diff = b - a;

                let node1 = a - diff;
                let node2 = b + diff;

                if is_inside(node1, rows, cols) {
                    antinodes.insert(node1);
                }
                if is_inside(node2, rows, cols) {
                    antinodes.insert(node2);
                }
            }
        }
    }
    antinodes.len()
}

fn is_inside(node: IVec2, rows: i32, cols: i32) -> bool {
    node.x >= 0 && node.y >= 0 && node.x < cols && node.y < rows
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        assert_eq!(process(input), 14);
    }
}
