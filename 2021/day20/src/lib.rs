pub mod parsing;
pub mod part1;
pub mod part2;

fn find_edge(
    image: &[Vec<char>],
    fill_char: char,
    (min_r, max_r, min_c, max_c): (usize, usize, usize, usize),
) -> (usize, usize, usize, usize) {
    let top = (min_r..max_r)
        .find(|r| image[*r][min_c..max_c].iter().any(|x| *x != fill_char))
        .unwrap();
    let bottom = (min_r..max_r)
        .rfind(|r| image[*r][min_c..max_c].iter().any(|x| *x != fill_char))
        .unwrap();
    let left = (min_c..max_c)
        .find(|i| {
            image[top..=bottom]
                .iter()
                .map(|r| r[*i])
                .any(|x| x != fill_char)
        })
        .unwrap();
    let right = (min_c..max_c)
        .rfind(|i| {
            image[top..=bottom]
                .iter()
                .map(|r| r[*i])
                .any(|x| x != fill_char)
        })
        .unwrap();
    (top, bottom, left, right)
}

fn expand(image: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (top, bottom, left, right) = find_edge(&image, '.', (0, image.len(), 0, image[0].len()));
    let col = right - left + 1;
    let mut new_image = vec![vec!['.'; col + 8]; 4];
    for r in image.into_iter().skip(top).take(bottom - top + 1) {
        let mut v = vec!['.'; 4];
        v.extend(r.into_iter().skip(left).take(right - left + 1));
        v.extend(['.'; 4]);
        new_image.push(v);
    }

    new_image.extend(vec![vec!['.'; col + 8]; 4]);
    new_image
}
fn collapse(image: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (top, bottom, left, right) =
        find_edge(&image, '#', (1, image.len() - 1, 1, image[0].len() - 1));
    image
        .into_iter()
        .skip(top.saturating_sub(2))
        .take(bottom - top + 5)
        .map(|row| {
            row.into_iter()
                .skip(left.saturating_sub(2))
                .take(right - left + 5)
                .collect()
        })
        .collect()
}

pub fn print_image(image: &Vec<Vec<char>>) {
    for row in image {
        for cell in row {
            print!("{cell}");
        }
        println!();
    }
}

pub fn run(algo: Vec<char>, mut image: Vec<Vec<char>>, i: usize) -> usize {
    for i in 0..i {
        image = if i % 2 == 0 {
            expand(image)
        } else {
            collapse(image)
        };
        print_image(&image);
        let row = image.len();
        let col = image[0].len();
        let mut new_image = vec![vec!['.'; col]; row];
        for r in 1..row - 1 {
            let (mut top, mut mid, mut bot) = (0, 0, 0);
            top = (top << 1) | usize::from(image[r - 1][0] == '#');
            top = (top << 1) | usize::from(image[r - 1][1] == '#');
            mid = (mid << 1) | usize::from(image[r][0] == '#');
            mid = (mid << 1) | usize::from(image[r][1] == '#');
            bot = (bot << 1) | usize::from(image[r + 1][0] == '#');
            bot = (bot << 1) | usize::from(image[r + 1][1] == '#');
            for c in 1..col - 1 {
                top = ((top << 1) | usize::from(image[r - 1][c + 1] == '#')) & 0b111;
                mid = ((mid << 1) | usize::from(image[r][c + 1] == '#')) & 0b111;
                bot = ((bot << 1) | usize::from(image[r + 1][c + 1] == '#')) & 0b111;
                let index = (((top << 3) | mid) << 3) | bot;
                new_image[r][c] = algo[index];
            }
        }
        image = new_image;
    }
    image
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|x| *x == '#')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::parse_input;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#
    }

    #[rstest]
    #[case(2, 35)]
    #[case(50, 3351)]
    fn test_run(fixture: &str, #[case] i: usize, #[case] expect: usize) {
        let (algo, image) = parse_input(fixture);
        assert_eq!(run(algo, image, i), expect);
    }
}
