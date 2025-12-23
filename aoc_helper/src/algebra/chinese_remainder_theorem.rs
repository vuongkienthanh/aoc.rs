#[derive(Debug)]
pub struct Item {
    pub modulus: usize,
    pub rem: usize,
}
pub fn extended_gcd(a: usize, b: usize) -> (usize, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }

    let q = (b / a) as i64;
    let (gcd, x1, y1) = extended_gcd(b % a, a);
    let x = y1 - q * x1;
    let y = x1;
    (gcd, x, y)
}

pub fn modular_inverse(a: usize, m: usize) -> Option<usize> {
    if m <= 1 {
        return None;
    }

    let a_mod = a % m;
    let (gcd, mut x, _) = extended_gcd(a_mod, m);

    if gcd != 1 {
        return None;
    }

    let m_i64 = m as i64;
    x = (x % m_i64 + m_i64) % m_i64;
    Some(x as usize)
}

pub fn find_x(input: Vec<Item>) -> usize {
    println!("Chinese Remainder Theorem:");
    for i in &input {
        println!("x = {} (mod {})", i.rem, i.modulus);
    }

    let modulus_product = input.iter().map(|x| x.modulus).product::<usize>();
    let identities = input
        .iter()
        .map(|x| modulus_product / x.modulus)
        .collect::<Vec<usize>>();
    let ys = input
        .iter()
        .zip(identities.iter())
        .map(|(x, y)| modular_inverse(*y, x.modulus).unwrap())
        .collect::<Vec<usize>>();

    println!("\nM = {modulus_product}\n");
    for (i, id) in identities.iter().enumerate() {
        println!("M{i} = {id}");
    }
    println!();
    for (i, v) in ys.iter().enumerate() {
        println!("y{i} = {v}");
    }
    println!();

    input
        .into_iter()
        .zip(identities)
        .zip(ys)
        .map(|((a, b), c)| a.rem * b * c)
        .sum::<usize>()
        % modulus_product
}
