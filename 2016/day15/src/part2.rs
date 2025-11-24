use crate::modular_inverse;
use crate::parsing::{Item, parse_input};

pub fn process(_input: &str) -> usize {
    let (_, mut input) = parse_input(_input).unwrap();
    input.push(Item {
        modulus: 11,
        rem: 11 - (input.len() + 1),
    });
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
