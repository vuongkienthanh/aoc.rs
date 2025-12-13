pub mod parsing;
pub mod part1;
pub mod part2;

fn extended_gcd(a: usize, b: usize) -> (usize, i64, i64) {
    if a == 0 {
        // Base case: gcd(0, b) = b.
        // The equation is 0*x + b*y = b, satisfied by x=0, y=1.
        return (b, 0, 1);
    }

    // Recursive step: extended_gcd(b % a, a)
    // q = b / a
    let q = (b / a) as i64;

    // Solve for (b % a) * x1 + a * y1 = gcd
    let (gcd, x1, y1) = extended_gcd(b % a, a);

    // Back-substitute to find x and y for a*x + b*y = gcd
    // x = y1 - q * x1
    // y = x1
    let x = y1 - q * x1;
    let y = x1;

    (gcd, x, y)
}

fn modular_inverse(a: usize, m: usize) -> Option<usize> {
    if m <= 1 {
        // Inverse is not meaningfully defined for m=0 or m=1
        return None;
    }

    // Ensure 'a' is reduced modulo 'm'.
    let a_mod = a % m;

    // Use the Extended Euclidean Algorithm to solve a*x + m*y = gcd(a, m).
    // x is returned as i64, which may be negative.
    let (gcd, mut x, _) = extended_gcd(a_mod, m);

    // The inverse exists only if gcd(a, m) = 1.
    if gcd != 1 {
        return None;
    }

    // Convert 'm' to i64 for calculation, as 'x' is i64 and we need to handle signed math.
    let m_i64 = m as i64;

    // Convert 'x' to the canonical positive residue in the range [0, m-1).
    // The expression (x % m_i64 + m_i64) % m_i64 handles negative results correctly.
    x = (x % m_i64 + m_i64) % m_i64;

    // Cast the non-negative result back to usize. This is safe because x < m, and m is usize.
    Some(x as usize)
}
