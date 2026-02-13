pub mod parsing;
pub mod part1;
pub mod part2;
use fxhash::{FxHashMap, FxHashSet};

fn allergen_list<'a>(input: &[(Vec<&'a str>, Vec<&'a str>)]) -> FxHashMap<&'a str, &'a str> {
    let mut list: FxHashMap<&'a str, FxHashSet<&'a str>> = FxHashMap::default();

    for (ingredients, allergens) in input {
        let ingredients = FxHashSet::from_iter(ingredients.into_iter().cloned());
        for a in allergens {
            list.entry(a)
                .and_modify(|v| *v = v.intersection(&ingredients).cloned().collect())
                .or_insert(ingredients.clone());
        }
    }

    while list.values().any(|v| v.len() > 1) {
        let ones: Vec<&str> = list
            .values()
            .filter(|v| v.len() == 1)
            .map(|v| v.iter().next().cloned().unwrap())
            .collect();
        for set in list.values_mut() {
            if set.len() != 1 {
                set.retain(|x| !ones.contains(x));
            }
        }
    }
    assert!(list.values().all(|x| x.len() == 1));

    list.into_iter()
        .map(|(k, v)| (k, v.into_iter().next().unwrap()))
        .collect()
}
