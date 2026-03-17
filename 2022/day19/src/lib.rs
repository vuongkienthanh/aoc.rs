pub mod parsing;
pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Resource {
    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: usize,
    geode_robot: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}
impl Resource {
    fn step(&mut self) {
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obsidian += self.obsidian_robot;
        self.geode += self.geode_robot;
    }
    // fn merge(&self, other: &Self) -> Option<Resource> {
    //     if self.ore_robot == other.ore_robot
    //         && self.clay_robot == other.clay_robot
    //         && self.obsidian_robot == other.obsidian_robot
    //         && self.geode_robot == other.geode_robot
    //     {
    //         if self.ore <= other.ore
    //             && self.clay <= other.clay
    //             && self.obsidian <= other.obsidian
    //             && self.geode <= other.geode
    //         {
    //             Some(other.clone())
    //         } else if self.ore >= other.ore
    //             && self.clay >= other.clay
    //             && self.obsidian >= other.obsidian
    //             && self.geode >= other.geode
    //         {
    //             Some(self.clone())
    //         } else {
    //             None
    //         }
    //     } else {
    //         None
    //     }
    // }
}

#[derive(Debug)]
pub struct Blueprint {
    id: usize,
    ore_robot_need_ore: usize,
    clay_robot_need_ore: usize,
    obsidian_robot_need_ore: usize,
    obsidian_robot_need_clay: usize,
    geode_robot_need_ore: usize,
    geode_robot_need_obsidian: usize,
}
impl Blueprint {
    fn can_make_ore_robot(&self, resource: &Resource) -> bool {
        resource.ore >= self.ore_robot_need_ore
    }
    fn make_ore_robot(&self, resource: &Resource) -> Resource {
        let mut new_resource = resource.clone();
        new_resource.ore -= self.ore_robot_need_ore;
        new_resource.step();
        new_resource.ore_robot += 1;
        new_resource
    }
    fn can_make_clay_robot(&self, resource: &Resource) -> bool {
        resource.ore >= self.clay_robot_need_ore
    }
    fn make_clay_robot(&self, resource: &Resource) -> Resource {
        let mut new_resource = resource.clone();
        new_resource.ore -= self.clay_robot_need_ore;
        new_resource.step();
        new_resource.clay_robot += 1;
        new_resource
    }
    fn can_make_obsidian_robot(&self, resource: &Resource) ->bool {
        resource.ore >= self.obsidian_robot_need_ore
        && resource.clay >= self.obsidian_robot_need_clay
    }
    fn make_obsidian_robot(&self, resource: &Resource) -> Resource {
        let mut new_resource = resource.clone();
        new_resource.ore -= self.obsidian_robot_need_ore;
        new_resource.clay -= self.obsidian_robot_need_clay;
        new_resource.step();
        new_resource.obsidian_robot += 1;
        new_resource
    }
    fn can_make_geode_robot(&self, resource: &Resource) -> bool {
        resource.ore >= self.geode_robot_need_ore
        && resource.obsidian >= self.geode_robot_need_obsidian
    }
    fn make_geode_robot(&self, resource: &Resource) -> Resource {
        let mut new_resource = resource.clone();
        new_resource.ore -= self.geode_robot_need_ore;
        new_resource.obsidian -= self.geode_robot_need_obsidian;
        new_resource.step();
        new_resource.geode_robot += 1;
        new_resource
    }
}

// pub fn process(_input: &str) -> usize {
//     let input = parse_input(_input);
//     let mut ans = 0;
//
//     for bp in input {
//         let mut current = vec![Resource {
//             ore_robot: 1,
//             clay_robot: 0,
//             obsidian_robot: 0,
//             geode_robot: 0,
//             ore: 0,
//             clay: 0,
//             obsidian: 0,
//             geode: 0,
//             time: 24,
//         }];
//         let max = current.into_iter().map(|r| r.geode).max().unwrap();
//         println!("id={},  max={max}", bp.id);
//         ans += bp.id * max;
//         panic!()
//     }
//     ans
// }

// pub fn merge(v: Vec<Resource>) -> Vec<Resource> {
//     let mut new = vec![];
//     'a: for a in v {
//         for b in new.iter_mut() {
//             if let Some(r) = a.merge(&*b) {
//                 *b = r;
//                 continue 'a;
//             }
//         }
//         new.push(a);
//     }
//     new
// }

