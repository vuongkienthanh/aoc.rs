pub type Range = (isize, isize);
pub type Cube = (Range, Range, Range);

pub fn try_intersect_range((a0, a1): Range, (b0, b1): Range) -> Option<Range> {
    if b1 < a0 || b0 > a1 {
        None
    } else {
        Some((a0.max(b0), a1.min(b1)))
    }
}

pub fn try_intersect_cube((a, b, c): Cube, (x, y, z): Cube) -> Option<Cube> {
    match (
        try_intersect_range(a, x),
        try_intersect_range(b, y),
        try_intersect_range(c, z),
    ) {
        (Some(d), Some(e), Some(f)) => Some((d, e, f)),
        _ => None,
    }
}

/// assume a and b is intersected and a >b
pub fn cut_cube(
    ((a0, a1), (b0, b1), (c0, c1)): Cube,
    ((x0, x1), (y0, y1), (z0, z1)): Cube,
) -> Vec<Cube> {
    [
        ////
        ((a0, x0 - 1), (b0, y0 - 1), (c0, z0 - 1)),
        ((a0, x0 - 1), (b0, y0 - 1), (z0, z1)),
        ((a0, x0 - 1), (b0, y0 - 1), (z1 + 1, c1)),
        //
        ((a0, x0 - 1), (y0, y1), (c0, z0 - 1)),
        ((a0, x0 - 1), (y0, y1), (z0, z1)),
        ((a0, x0 - 1), (y0, y1), (z1 + 1, c1)),
        //
        ((a0, x0 - 1), (y1 + 1, b1), (c0, z0 - 1)),
        ((a0, x0 - 1), (y1 + 1, b1), (z0, z1)),
        ((a0, x0 - 1), (y1 + 1, b1), (z1 + 1, c1)),
        ////
        ((x0, x1), (b0, y0 - 1), (c0, z0 - 1)),
        ((x0, x1), (b0, y0 - 1), (z0, z1)),
        ((x0, x1), (b0, y0 - 1), (z1 + 1, c1)),
        //
        ((x0, x1), (y0, y1), (c0, z0 - 1)),
        // ((x0, x1), (y0, y1), (z0, z1)),
        ((x0, x1), (y0, y1), (z1 + 1, c1)),
        //
        ((x0, x1), (y1 + 1, b1), (c0, z0 - 1)),
        ((x0, x1), (y1 + 1, b1), (z0, z1)),
        ((x0, x1), (y1 + 1, b1), (z1 + 1, c1)),
        ////
        ((x1 + 1, a1), (b0, y0 - 1), (c0, z0 - 1)),
        ((x1 + 1, a1), (b0, y0 - 1), (z0, z1)),
        ((x1 + 1, a1), (b0, y0 - 1), (z1 + 1, c1)),
        //
        ((x1 + 1, a1), (y0, y1), (c0, z0 - 1)),
        ((x1 + 1, a1), (y0, y1), (z0, z1)),
        ((x1 + 1, a1), (y0, y1), (z1 + 1, c1)),
        //
        ((x1 + 1, a1), (y1 + 1, b1), (c0, z0 - 1)),
        ((x1 + 1, a1), (y1 + 1, b1), (z0, z1)),
        ((x1 + 1, a1), (y1 + 1, b1), (z1 + 1, c1)),
    ]
    .into_iter()
    .flat_map(|((a0, a1), (b0, b1), (c0, c1))| {
        (a0 <= a1 && b0 <= b1 && c0 <= c1).then_some(((a0, a1), (b0, b1), (c0, c1)))
    })
    .collect()
}
