pub mod parsing;
pub mod part1;
use parsing::Point;

fn step(p: &[Point], v: &[Point], i: usize) -> Vec<Point> {
    p.iter()
        .zip(v)
        .map(|(p, v)| (p.0 + i as isize * v.0, p.1 + i as isize * v.1))
        .collect()
}

fn diffuseness(p: &[Point]) -> usize {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (isize::MAX, isize::MAX, 0, 0);
    for c in p {
        min_x = min_x.min(c.0);
        min_y = min_y.min(c.1);
        max_x = max_x.max(c.0);
        max_y = max_y.max(c.1);
    }
    ( (max_x - min_x) * (max_y - min_y) ) as usize
}

fn display(p: &[Point]) {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (isize::MAX, isize::MAX, 0, 0);
    for c in p {
        min_x = min_x.min(c.0);
        min_y = min_y.min(c.1);
        max_x = max_x.max(c.0);
        max_y = max_y.max(c.1);
    }
    max_x -= min_x;
    max_y -= min_y;
    let mut grid = vec![vec!['.'; max_x as usize + 1]; max_y as usize + 1];
    for c in p {
        grid[(c.1 - min_y) as usize][(c.0 - min_x) as usize] = '#';
    }

    for row in grid.into_iter() {
        for cell in row.into_iter() {
            print!("{cell}");
        }
        println!();
    }
}
