use std::{
    collections::{HashMap, HashSet},
    fs,
    sync::{Arc, Mutex},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};

use itertools::Itertools;
use pathfinding::directed::{dfs::dfs_reach, dijkstra::dijkstra};
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    tunnels: Vec<(&'a str, u32)>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Node<'a> {
    depth: u32,
    valve: &'a Valve<'a>,
    open_valves: Vec<&'a str>,
    pressure: u32,
    pressure_per_minute: u32,
}

fn parse_valves(input: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(
        line_ending,
        tuple((
            tag("Valve "),
            alpha1,
            tag(" has flow rate="),
            complete::u32,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), alpha1.map(|tunnel| (tunnel, 1 as u32))),
        ))
        .map(|(_, name, _, flow_rate, _, tunnels)| Valve {
            name,
            flow_rate,
            tunnels: tunnels.into_iter().collect(),
        }),
    )(input)
}

fn next_nodes<'a>(
    node: &Node<'a>,
    valves: &'a HashMap<&'a str, &'a Valve<'a>>,
    working_valve_count: usize,
    max_depth: u32,
) -> Vec<Node<'a>> {
    if node.depth >= max_depth || node.open_valves.len() == working_valve_count {
        return vec![];
    }
    let mut next_nodes = vec![];

    //open current valve
    if node.valve.flow_rate > 0 && !node.open_valves.contains(&node.valve.name) {
        let mut open_valves = node.open_valves.clone();
        open_valves.push(node.valve.name);
        let waiting = open_valves.len() == working_valve_count;
        next_nodes.push(Node {
            depth: node.depth + 1,
            valve: node.valve,
            open_valves,
            pressure: node.pressure + node.pressure_per_minute,
            pressure_per_minute: node.pressure_per_minute + node.valve.flow_rate,
        });
        if waiting {
            return next_nodes;
        }
    }

    //move to next valves
    node.valve
        .tunnels
        .iter()
        .filter(|(tunnel, _)| !node.open_valves.contains(tunnel))
        .for_each(|(tunnel, distance)| {
            if let Some(valve) = valves.get(tunnel) {
                let depth = node.depth + distance;
                if depth <= max_depth {
                    next_nodes.push(Node {
                        depth,
                        valve,
                        open_valves: node.open_valves.clone(),
                        pressure: node.pressure + distance * node.pressure_per_minute,
                        pressure_per_minute: node.pressure_per_minute,
                    });
                }
            }
        });

    next_nodes
}

fn main() {
    let input = fs::read_to_string("src/day16/input.txt").unwrap();
    let (_, valves) = parse_valves(&input).unwrap();

    let valves_by_name = valves
        .iter()
        .map(|valve| (valve.name, valve))
        .collect::<HashMap<&str, &Valve>>();

    let starting_valve = valves_by_name["AA"];

    let interesting_valve_names = valves
        .iter()
        .filter(|valve| valve == &starting_valve || valve.flow_rate > 0)
        .map(|valve| valve.name)
        .collect::<HashSet<&str>>();

    let interesting_valves = interesting_valve_names
        .iter()
        .map(|valve| {
            let tunnels = interesting_valve_names
                .iter()
                .filter(|other_valve| other_valve != &valve)
                .filter_map(|other_valve| {
                    let path = dijkstra(
                        valve,
                        |valve| valves_by_name[valve].tunnels.clone(),
                        |valve| valve == other_valve,
                    );
                    if let Some((_, distance)) = path {
                        Some((*other_valve, distance))
                    } else {
                        print!("No path found");
                        None
                    }
                })
                .collect::<Vec<(&str, u32)>>();
            let current_valve = valves_by_name[*valve];
            Valve {
                name: current_valve.name,
                flow_rate: current_valve.flow_rate,
                tunnels,
            }
        })
        .collect::<Vec<Valve>>();

    let working_valve_count = interesting_valves
        .iter()
        .filter(|valve| valve.flow_rate > 0)
        .count();
    let interesing_valves_by_name = interesting_valves
        .iter()
        .map(|valve| (valve.name, valve))
        .collect::<HashMap<&str, &Valve>>();

    let starting_valve = interesing_valves_by_name[starting_valve.name];

    let starting_node = Node {
        depth: 0,
        valve: starting_valve,
        open_valves: vec![],
        pressure: 0,
        pressure_per_minute: 0,
    };

    let max_depth = 30;

    let steps = dfs_reach(starting_node, |node| {
        next_nodes(
            node,
            &interesing_valves_by_name,
            working_valve_count,
            max_depth,
        )
    });

    let mut max = 0;

    steps.into_iter().for_each(|step| {
        let pressure = (step.pressure + (max_depth - step.depth) * step.pressure_per_minute) as i32;
        if pressure > max {
            max = pressure;
            println!("Current max: {}: {:?}", max, step.open_valves);
        }
    });

    println!("Part 1: {}", max);

    let max_depth = 26;
    let abosulute_max = Arc::new(Mutex::new(0));

    interesting_valve_names
        .iter()
        .filter(|name| **name != "AA")
        .cloned()
        .combinations((interesting_valve_names.len() - 1) / 2)
        .par_bridge()
        .for_each(|my_valve_names| {
            let elephant_valve_names = interesting_valve_names
                .iter()
                .cloned()
                .filter(|name| *name != "AA" && !my_valve_names.contains(name))
                .collect::<Vec<&str>>();

            let my_valves_by_name = interesing_valves_by_name
                .iter()
                .filter(|(k, _)| **k == "AA" || my_valve_names.contains(*k))
                .map(|(k, v)| (*k, *v))
                .collect::<HashMap<&str, &Valve>>();
            let elephant_valves_by_name = interesing_valves_by_name
                .iter()
                .filter(|(k, _)| **k == "AA" || elephant_valve_names.contains(*k))
                .map(|(k, v)| (*k, *v))
                .collect::<HashMap<&str, &Valve>>();

            let starting_node = Node {
                depth: 0,
                valve: starting_valve,
                open_valves: vec![],
                pressure: 0,
                pressure_per_minute: 0,
            };

            let my_working_valve_count = my_valves_by_name.len() - 1;

            let steps = dfs_reach(starting_node, |node| {
                next_nodes(node, &my_valves_by_name, my_working_valve_count, max_depth)
            });

            let mut my_max = 0;
            let mut my_path = vec![];

            steps.into_iter().for_each(|step| {
                let pressure =
                    (step.pressure + (max_depth - step.depth) * step.pressure_per_minute) as i32;
                if pressure > my_max {
                    my_max = pressure;
                    my_path = step.open_valves;
                }
            });

            let starting_node = Node {
                depth: 0,
                valve: starting_valve,
                open_valves: vec![],
                pressure: 0,
                pressure_per_minute: 0,
            };

            let elephant_working_valve_count = elephant_valves_by_name.len() - 1;

            let steps = dfs_reach(starting_node, |node| {
                next_nodes(
                    node,
                    &elephant_valves_by_name,
                    elephant_working_valve_count,
                    max_depth,
                )
            });

            let mut elephant_max = 0;
            let mut elephant_path = vec![];

            steps.into_iter().for_each(|step| {
                let pressure =
                    (step.pressure + (max_depth - step.depth) * step.pressure_per_minute) as i32;
                if pressure > elephant_max {
                    elephant_max = pressure;
                    elephant_path = step.open_valves;
                }
            });

            let max = my_max + elephant_max;
            let abosulute_max = Arc::clone(&abosulute_max);
            let mut abosulute_max_value = abosulute_max.lock().unwrap();
            if max > *abosulute_max_value {
                *abosulute_max_value = max;
                println!("Current max: {}", max);
                println!("My path: {:?}", my_path);
                println!("Elephant path: {:?}", elephant_path);
            }
        });

    println!("Part 2: {}", *abosulute_max.lock().unwrap());
}
