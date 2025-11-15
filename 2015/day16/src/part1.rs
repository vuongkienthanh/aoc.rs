use crate::TICKER_TAPE;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (_rest, mut aunt_list) = parse_input(_input).unwrap();
    assert!(_rest.is_empty());

    while let Some((aunt, prop)) = aunt_list.pop() {
        if prop.into_iter().zip(TICKER_TAPE).all(|(p, t)| match p {
            None => true,
            Some(i) => i == t,
        }) {
            return aunt;
        }
    }

    panic!("can't find answer")
}
