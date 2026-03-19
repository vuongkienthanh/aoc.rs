pub mod parsing;
pub mod part1;
pub mod part2;

fn last_chance_cal(x: usize) -> usize {
    match x {
        1 => 2,
        2..=3 => 3,
        4..=6 => 4,
        7..=10 => 5,
        11..=15 => 6,
        16..=21 => 7,
        _ => panic!(),
    }
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub ore_robot: usize,
    pub clay_robot: usize,
    pub obsidian_robot: usize,
    pub geode_robot: usize,
    pub ore: usize,
    pub clay: usize,
    pub obsidian: usize,
    pub geode: usize,
}
impl Resource {
    fn new() -> Self {
        Resource {
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
    fn step(&mut self) {
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obsidian += self.obsidian_robot;
        self.geode += self.geode_robot;
    }
}

#[derive(Debug)]
pub struct Blueprint {
    pub id: usize,
    pub ore_robot_need_ore: usize,
    pub clay_robot_need_ore: usize,
    pub obsidian_robot_need_ore: usize,
    pub obsidian_robot_need_clay: usize,
    pub geode_robot_need_ore: usize,
    pub geode_robot_need_obsidian: usize,
    pub max_ore_robot: usize,
    pub last_chance_build_obsidian: usize,
    pub last_chance_build_clay: usize,
}
impl Blueprint {
    fn new(
        id: usize,
        ore_robot_need_ore: usize,
        clay_robot_need_ore: usize,
        obsidian_robot_need_ore: usize,
        obsidian_robot_need_clay: usize,
        geode_robot_need_ore: usize,
        geode_robot_need_obsidian: usize,
    ) -> Self {
        let last_chance_build_obsidian = 22 - last_chance_cal(geode_robot_need_obsidian);
        let last_chance_build_clay =
            last_chance_build_obsidian - last_chance_cal(obsidian_robot_need_clay);
        Self {
            id,
            ore_robot_need_ore,
            clay_robot_need_ore,
            obsidian_robot_need_ore,
            obsidian_robot_need_clay,
            geode_robot_need_ore,
            geode_robot_need_obsidian,
            max_ore_robot: [
                ore_robot_need_ore,
                clay_robot_need_ore,
                obsidian_robot_need_ore,
                geode_robot_need_ore,
            ]
            .into_iter()
            .max()
            .unwrap(),
            last_chance_build_obsidian,
            last_chance_build_clay,
        }
    }
    fn can_make_ore_robot(&self, resource: &Resource) -> bool {
        resource.ore_robot < self.max_ore_robot && resource.ore >= self.ore_robot_need_ore
    }
    fn make_ore_robot(&self, resource: &Resource) -> Resource {
        let mut new_resource = resource.clone();
        new_resource.ore -= self.ore_robot_need_ore;
        new_resource.step();
        new_resource.ore_robot += 1;
        new_resource
    }
    fn can_make_clay_robot(&self, resource: &Resource) -> bool {
        resource.clay_robot < self.obsidian_robot_need_clay
            && resource.ore >= self.clay_robot_need_ore
    }
    fn make_clay_robot(&self, resource: &Resource) -> Resource {
        let mut new_resource = resource.clone();
        new_resource.ore -= self.clay_robot_need_ore;
        new_resource.step();
        new_resource.clay_robot += 1;
        new_resource
    }
    fn can_make_obsidian_robot(&self, resource: &Resource) -> bool {
        resource.ore >= self.obsidian_robot_need_ore
            && resource.clay >= self.obsidian_robot_need_clay
            && resource.obsidian_robot < self.geode_robot_need_obsidian
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
