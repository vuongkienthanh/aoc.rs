use crate::parsing::parse_input;
use crate::allergen_list;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let allergens : Vec<&str> = allergen_list(&input).into_values().collect();
    input.into_iter().map(|(x, _)| x).flatten().filter(|x| !allergens.contains(x)).count()
}
