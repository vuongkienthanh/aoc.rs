use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let map = parse_input(_input);
    // println!("{input:?}");
    let mut max = 0;
    let mut current = vec![(vec!["AA"], 0)];
    // for i in (2..=30).step_by(2) {
    //     let mut new = vec![];
    //
    //     for (path, mut pressure) in current {
    //         let last = path.last().unwrap();
    //         let (last_pressure, last_path) = map.get(last).unwrap();
    //         pressure += last_pressure * (32 - i);
    //         max = max.max(pressure);
    //         for next_loc in last_path {
    //             let mut path = path.clone();
    //             path.push(next_loc);
    //             new.push((path, pressure));
    //         }
    //     }
    //
    //     current = new;
    //     println!("{}", current.len());
    // }
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 1651);
    }
}
