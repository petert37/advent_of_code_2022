use std::{
    fs,
    ops::{Add, Sub},
};

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};
use pathfinding::prelude::dfs_reach;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Resources {
    ore: u16,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

impl Add for &Resources {
    type Output = Resources;

    fn add(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore.checked_add(rhs.ore).unwrap(),
            clay: self.clay.checked_add(rhs.clay).unwrap_or(250),
            obsidian: self.obsidian.checked_add(rhs.obsidian).unwrap_or(250),
            geode: self.geode.checked_add(rhs.geode).unwrap_or(250),
        }
    }
}

impl Sub for &Resources {
    type Output = Resources;

    fn sub(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore.checked_sub(rhs.ore).unwrap(),
            clay: self.clay.checked_sub(rhs.clay).unwrap(),
            obsidian: self.obsidian.checked_sub(rhs.obsidian).unwrap(),
            geode: self.geode.checked_sub(rhs.geode).unwrap(),
        }
    }
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else if self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
        {
            Some(std::cmp::Ordering::Greater)
        } else if self.ore <= other.ore
            && self.clay <= other.clay
            && self.obsidian <= other.obsidian
            && self.geode <= other.geode
        {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Robot {
    resource_production: Resources,
    building_cost: Resources,
}

#[derive(Debug)]
struct Blueprint {
    id: u8,
    robots: [Robot; 4],
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Node {
    time: u8,
    resources: Resources,
    resource_production: Resources,
}

fn parse_blueprints(input: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(
        line_ending,
        tuple((
            tag("Blueprint "),
            complete::u8,
            tag(": Each ore robot costs "),
            complete::u16,
            tag(" ore. Each clay robot costs "),
            complete::u8,
            tag(" ore. Each obsidian robot costs "),
            complete::u8,
            tag(" ore and "),
            complete::u8,
            tag(" clay. Each geode robot costs "),
            complete::u8,
            tag(" ore and "),
            complete::u8,
            tag(" obsidian."),
        ))
        .map(
            |(
                _,
                id,
                _,
                ore_ore,
                _,
                clay_ore,
                _,
                obsidian_ore,
                _,
                obsidian_clay,
                _,
                geode_ore,
                _,
                geode_obsidian,
                _,
            )| Blueprint {
                id,
                robots: [
                    Robot {
                        resource_production: Resources {
                            ore: 1,
                            clay: 0,
                            obsidian: 0,
                            geode: 0,
                        },
                        building_cost: Resources {
                            ore: ore_ore,
                            clay: 0,
                            obsidian: 0,
                            geode: 0,
                        },
                    },
                    Robot {
                        resource_production: Resources {
                            ore: 0,
                            clay: 1,
                            obsidian: 0,
                            geode: 0,
                        },
                        building_cost: Resources {
                            ore: clay_ore as u16,
                            clay: 0,
                            obsidian: 0,
                            geode: 0,
                        },
                    },
                    Robot {
                        resource_production: Resources {
                            ore: 0,
                            clay: 0,
                            obsidian: 1,
                            geode: 0,
                        },
                        building_cost: Resources {
                            ore: obsidian_ore as u16,
                            clay: obsidian_clay,
                            obsidian: 0,
                            geode: 0,
                        },
                    },
                    Robot {
                        resource_production: Resources {
                            ore: 0,
                            clay: 0,
                            obsidian: 0,
                            geode: 1,
                        },
                        building_cost: Resources {
                            ore: geode_ore as u16,
                            clay: 0,
                            obsidian: geode_obsidian,
                            geode: 0,
                        },
                    },
                ],
            },
        ),
    )(input)
}

fn next_nodes_1(node: &Node, blueprint: &Blueprint, max_time: u8) -> Vec<Node> {
    let mut result = vec![];
    if node.time >= max_time {
        return result;
    }

    let time = node.time + 1;
    let new_resources = &node.resources + &node.resource_production;

    for robot_blueprint in blueprint.robots[1..4].iter().rev() {
        if &robot_blueprint.building_cost <= &node.resources {
            result.push(Node {
                time,
                resources: &new_resources - &robot_blueprint.building_cost,
                resource_production: &node.resource_production
                    + &robot_blueprint.resource_production,
            });
            break;
        }
    }

    let ore_robot_blueprint = &blueprint.robots[0];
    if &ore_robot_blueprint.building_cost <= &node.resources {
        result.push(Node {
            time,
            resources: &new_resources - &ore_robot_blueprint.building_cost,
            resource_production: &node.resource_production
                + &ore_robot_blueprint.resource_production,
        });
    }

    result.push(Node {
        time,
        resources: new_resources,
        resource_production: node.resource_production,
    });

    result
}

fn next_nodes_2(node: &Node, blueprint: &Blueprint, max_time: u8) -> Vec<Node> {
    let mut result = vec![];
    if node.time >= max_time {
        return result;
    }

    let time = node.time + 1;
    let new_resources = &node.resources + &node.resource_production;

    for robot_blueprint in blueprint.robots[1..4].iter().rev() {
        if &robot_blueprint.building_cost <= &node.resources {
            result.push(Node {
                time,
                resources: &new_resources - &robot_blueprint.building_cost,
                resource_production: &node.resource_production
                    + &robot_blueprint.resource_production,
            });
            break;
        }
    }

    let ore_robot_blueprint = &blueprint.robots[0];
    if &ore_robot_blueprint.building_cost <= &node.resources {
        result.push(Node {
            time,
            resources: &new_resources - &ore_robot_blueprint.building_cost,
            resource_production: &node.resource_production
                + &ore_robot_blueprint.resource_production,
        });
    }

    if !result.is_empty() {
        return result;
    }

    result.push(Node {
        time,
        resources: new_resources,
        resource_production: node.resource_production,
    });

    result
}

fn get_max_geodes<FN>(blueprint: &Blueprint, max_time: u8, mut next_nodes_fn: FN) -> u8
where
    FN: FnMut(&Node, &Blueprint, u8) -> Vec<Node>,
{
    let starting_node = Node {
        time: 0,
        resources: Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        resource_production: Resources {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
    };

    let steps = dfs_reach(starting_node, |node| {
        next_nodes_fn(node, blueprint, max_time)
    });

    let mut max = 0;
    for step in steps {
        if step.resources.geode > max {
            max = step.resources.geode;
        }
    }
    max
}

fn main() {
    let input = fs::read_to_string("src/day19/input.txt").unwrap();
    let (_, blueprints) = parse_blueprints(&input).unwrap();

    let mut sum_quality_level = 0;

    for blueprint in &blueprints {
        let geodes = get_max_geodes(blueprint, 24, next_nodes_1);
        println!("Blueprint {} geodes: {}", blueprint.id, geodes);
        let quality_level = blueprint.id as u32 * geodes as u32;
        sum_quality_level += quality_level;
    }

    println!("Sum quality level: {}", sum_quality_level);

    let mut sum_quality_level = 1;

    for blueprint in blueprints.iter().take(3) {
        let geodes = get_max_geodes(blueprint, 32, next_nodes_2);
        println!("Blueprint {} geodes: {}", blueprint.id, geodes);
        sum_quality_level *= geodes as u32;
    }

    println!("Sum quality level: {}", sum_quality_level);
}
