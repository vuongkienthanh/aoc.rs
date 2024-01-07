use std::collections::HashMap;
#[derive(PartialEq, Eq, Debug)]
enum Pulse {
    Low,
    High,
    Ignored,
}
impl Default for Pulse {
    fn default() -> Self {
        Pulse::Low
    }
}

type Target<'a> = Option<&'a str>;

trait Module {
    fn send<'a>(&'a mut self, receive: &Pulse) -> (Pulse, Target<'a>);
}

#[derive(Debug, Default)]
struct Flipflop {
    is_on: bool,
    target: String,
}
impl Module for Flipflop {
    fn send<'a>(&'a mut self, receive: &Pulse) -> (Pulse, Target<'a>) {
        assert_ne!(receive, &Pulse::Ignored);
        if receive == &Pulse::High {
            (Pulse::Ignored, None)
        } else {
            match self.is_on {
                true => {
                    self.is_on = false;
                    (Pulse::Low, Some(&self.target))
                }
                false => {
                    self.is_on = true;
                    (Pulse::High, Some(&self.target))
                }
            }
        }
    }
}
#[derive(Debug, Default)]
struct Conjunction {
    memory: HashMap<String, Pulse>,
    target: String,
}
impl Module for Conjunction {
    fn send<'a>(&'a mut self, receive: &Pulse) -> (Pulse, Target<'a>) {
        assert_ne!(receive, &Pulse::Ignored);
    }
}

pub fn process(_input: &str) -> usize {
    todo!("part1")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#""#;
        assert_eq!(process(input), 0);
    }
}
