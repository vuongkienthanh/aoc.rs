use crate::next_valid_password;

pub fn process(_input: &str) -> String {
    next_valid_password(_input.to_string())
}
