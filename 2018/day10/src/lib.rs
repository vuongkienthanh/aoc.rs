pub mod parsing;
pub mod part1;
use parsing::Point;

fn shrink_10000(p: &mut [Point], v: &[Point]) {
    for (a, b) in p.iter_mut().zip(v) {
        a.0 += b.0 * 10000;
        a.1 += b.1 * 10000;
    }
}

fn step(p: &[Point], v: &[Point]) -> Vec<Point> {
    p.iter()
        .zip(v)
        .map(|(p, v)| (p.0 + v.0, p.1 + v.1))
        .collect()
}

fn compactness(p: &[Point]) -> usize {
    let mut ans = 0;
    for i in 0..p.len() - 1 {
        for j in i + 1..p.len() {
            let (a, b) = (p[i], p[j]);
            if (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) == 1 {
                ans += 1;
            }
        }
    }
    ans
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
