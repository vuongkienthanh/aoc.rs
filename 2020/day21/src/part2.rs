use crate::allergen_list;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> String {
    let input = parse_input(_input);
    let mut allergens: Vec<(&str, &str)> = allergen_list(&input).into_iter().collect();
    allergens.sort_unstable_by_key(|(a, _)| *a);
    let allergens: Vec<&str> = allergens.into_iter().map(|(_, b)| b).collect();
    allergens.join(",")
}
