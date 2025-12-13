use crate::calc_area;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let row_map = parse_input(_input);
    // for (k, vy) in &row_map {
    //     println!("{k} {vy:?}");
    //     }
    // for (k, vx) in &col_map {
    //     println!("{k} {vx:?}");
    // }
    let rows: Vec<usize> = row_map.keys().cloned().collect();
    let mut max_area = 0;

    for i in 0..rows.len() - 1 {
        let top_row = rows[i];
        let top_range = row_map.get(&top_row).unwrap();

        for j in i + 1..rows.len() {
            let bottom_row = rows[j];
            let bottom_range = row_map.get(&bottom_row).unwrap();

            let top_left = (top_row, top_range.0);
            let bottom_right = (bottom_row, bottom_range.1);
            let area = calc_area(top_left, bottom_right);
            max_area = max_area.max(area);

            let top_right = (top_row, top_range.1);
            let bottom_left = (bottom_row, bottom_range.0);
            let area = calc_area(top_right, bottom_left);
            max_area = max_area.max(area);
        }
    }

    max_area
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 50);
    }
}
