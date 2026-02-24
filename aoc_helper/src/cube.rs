use std::collections::BTreeSet;
pub type Range = (isize, isize);
pub type Cube = (Range, Range, Range);

fn try_cube(((a0, a1), (b0, b1), (c0, c1)): Cube) -> Option<Cube> {
    if a0 > a1 || b0 > b1 || c0 > c1 {
        return None;
    }
    Some(((a0, a1), (b0, b1), (c0, c1)))
}

pub fn intersect_range((a0, a1): Range, (b0, b1): Range) -> Option<Range> {
    if b1 <= a0 || b0 >= a1 {
        None
    } else if b0 >= a0 && b1 <= a1 {
        Some((b0, b1))
    } else if b0 < a0 && b1 > a1 {
        Some((a0, a1))
    } else if b1 > a0 && b1 < a1 {
        Some((a0, b1))
    } else {
        Some((b0, a1))
    }
}

pub enum DivResult {
    LcontainsR,
    RcontainsL,
    NoIntersect,
    Intersect(Cube),
}
pub fn intersect_cube((a, b, c): Cube, (x, y, z): Cube) -> DivResult {
    match (
        intersect_range(a, x),
        intersect_range(b, y),
        intersect_range(c, z),
    ) {
        (None, _, _) | (_, None, _) | (_, _, None) => DivResult::NoIntersect,
        (d, e, f) => {
            let intersection = (d.unwrap(), e.unwrap(), f.unwrap());
            if intersection == (a, b, c) {
                DivResult::RcontainsL
            } else if intersection == (x, y, z) {
                DivResult::LcontainsR
            } else {
                DivResult::Intersect(intersection)
            }
        }
    }
}

/// assume a and b is intersected and a >b
pub fn cut_cube(
    ((a0, a1), (b0, b1), (c0, c1)): Cube,
    ((x0, x1), (y0, y1), (z0, z1)): Cube,
) -> Vec<Cube> {
    [
        ////
        try_cube(((a0, x0 - 1), (b0, y0 - 1), (c0, z0 - 1))),
        try_cube(((a0, x0 - 1), (b0, y0 - 1), (z0, z1))),
        try_cube(((a0, x0 - 1), (b0, y0 - 1), (z1 + 1, c1))),
        //
        try_cube(((a0, x0 - 1), (y0, y1), (c0, z0 - 1))),
        try_cube(((a0, x0 - 1), (y0, y1), (z0, z1))),
        try_cube(((a0, x0 - 1), (y0, y1), (z1 + 1, c1))),
        //
        try_cube(((a0, x0 - 1), (y1 + 1, b1), (c0, z0 - 1))),
        try_cube(((a0, x0 - 1), (y1 + 1, b1), (z0, z1))),
        try_cube(((a0, x0 - 1), (y1 + 1, b1), (z1 + 1, c1))),
        ////
        try_cube(((x0, x1), (b0, y0 - 1), (c0, z0 - 1))),
        try_cube(((x0, x1), (b0, y0 - 1), (z0, z1))),
        try_cube(((x0, x1), (b0, y0 - 1), (z1 + 1, c1))),
        //
        try_cube(((x0, x1), (y0, y1), (c0, z0 - 1))),
        // try_cube(((x0, x1), (y0, y1), (z0, z1))),
        try_cube(((x0, x1), (y0, y1), (z1 + 1, c1))),
        //
        try_cube(((x0, x1), (y1 + 1, b1), (c0, z0 - 1))),
        try_cube(((x0, x1), (y1 + 1, b1), (z0, z1))),
        try_cube(((x0, x1), (y1 + 1, b1), (z1 + 1, c1))),
        ////
        try_cube(((x1 + 1, a1), (b0, y0 - 1), (c0, z0 - 1))),
        try_cube(((x1 + 1, a1), (b0, y0 - 1), (z0, z1))),
        try_cube(((x1 + 1, a1), (b0, y0 - 1), (z1 + 1, c1))),
        //
        try_cube(((x1 + 1, a1), (y0, y1), (c0, z0 - 1))),
        try_cube(((x1 + 1, a1), (y0, y1), (z0, z1))),
        try_cube(((x1 + 1, a1), (y0, y1), (z1 + 1, c1))),
        //
        try_cube(((x1 + 1, a1), (y1 + 1, b1), (c0, z0 - 1))),
        try_cube(((x1 + 1, a1), (y1 + 1, b1), (z0, z1))),
        try_cube(((x1 + 1, a1), (y1 + 1, b1), (z1 + 1, c1))),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub fn merge(mut cubes: BTreeSet<Cube>) -> BTreeSet<Cube> {
    let mut v = vec![cubes.pop_last().unwrap()];

    for cube in cubes.into_iter().rev() {
        for i in 0..v.len() {
            let big_cube = v[i];
            if let DivResult::LcontainsR = intersect_cube(big_cube, cube) {
                continue;
            } else {
                v.push(cube);
                break;
            }
        }
    }

    v.into_iter().collect()
}
