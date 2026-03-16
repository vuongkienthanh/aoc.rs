pub mod part1;
pub mod part2;
pub mod parsing;

type Robots = (usize, usize, usize, usize);
type Resource = (usize, usize, usize, usize);

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot_need: usize,
    clay_robot_need: usize,
    obsidian_robot_need: (usize, usize),
    geode_robot_need: (usize, usize),
}
impl Blueprint {
    fn make_robot(&self, (a, b, c, d): Robots, resource: &Resource) -> Option<(Robots, Resource)> {
        if resource.ore >= self.geode_robot_need.0 && resource.obsidian >= self.geode_robot_need.1 {
            Some((
                (a, b, c, d + 1),
                Resource {
                    ore: resource.ore - self.geode_robot_need.0,
                    clay: resource.clay,
                    obsidian: resource.obsidian - self.geode_robot_need.1,
                    geode: resource.geode,
                },
            ));
        } else if resource.ore >= self.obsidian_robot_need.0
            && resource.clay >= self.obsidian_robot_need.1
        {
            Some((
                (a, b, c + 1, d),
                Resource {
                    ore: resource.ore - self.obsidian_robot_need.0,
                    clay: resource.clay - self.obsidian_robot_need.1,
                    obsidian: resource.obsidian,
                    geode: resource.geode,
                },
            ));
        } else if resource.ore >= self.clay_robot_need {
            Some((
                (a, b + 1, c, d),
                Resource {
                    ore: resource.ore - self.clay_robot_need,
                    clay: resource.clay,
                    obsidian: resource.obsidian,
                    geode: resource.geode,
                },
            ));
        } else if resource.ore >= self.ore_robot_need {
            Some((
                (a + 1, b, c, d),
                Resource {
                    ore: resource.ore - self.ore_robot_need,
                    clay: resource.clay,
                    obsidian: resource.obsidian,
                    geode: resource.geode,
                },
            ));
        } else {
            None
        }
    }
}