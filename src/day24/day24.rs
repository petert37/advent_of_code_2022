use std::fs;

use pathfinding::prelude::dfs_reach;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Space,
    Blizzard(Direction),
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    width: i32,
    height: i32,
}

impl Map {
    fn new(tiles: Vec<Vec<Tile>>) -> Map {
        let widht = tiles.get(0).map_or(0, |row| row.len() as i32);
        let height = tiles.len() as i32;
        Map {
            tiles,
            width: widht,
            height,
        }
    }

    fn check_position(&self, position: &Position, time: Time) -> bool {
        let x = position.0 as i32;
        let y = position.1 as i32;
        let time = time as i32;
        let modulo_width = self.width - 2;
        let modulo_height = self.height - 2;
        ![
            (
                (x - 1 - time).rem_euclid(modulo_width),
                y - 1,
                Direction::Right,
            ),
            (
                (x - 1 + time).rem_euclid(modulo_width),
                y - 1,
                Direction::Left,
            ),
            (
                x - 1,
                (y - 1 - time).rem_euclid(modulo_height),
                Direction::Down,
            ),
            (
                x - 1,
                (y - 1 + time).rem_euclid(modulo_height),
                Direction::Up,
            ),
        ]
        .into_iter()
        .any(|(x, y, direction)| {
            self.tiles
                .get((y + 1) as usize)
                .and_then(|row| row.get((x + 1) as usize))
                == Some(&Tile::Blizzard(direction))
        })
    }
}

type Position = (u8, u8);
type Time = u16;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Node {
    time: Time,
    position: Position,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Node2 {
    time: Time,
    position: Position,
    stage: u8,
}

fn parse_map(input: &str) -> Map {
    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Space,
                    '^' => Tile::Blizzard(Direction::Up),
                    '>' => Tile::Blizzard(Direction::Right),
                    'v' => Tile::Blizzard(Direction::Down),
                    '<' => Tile::Blizzard(Direction::Left),
                    _ => panic!("Invalid input"),
                })
                .collect()
        })
        .collect();
    Map::new(tiles)
}

fn positions_to_check(position: &Position, width: i32, height: i32) -> Vec<Position> {
    let x = position.0 as i32;
    let y = position.1 as i32;
    [(x, y + 1), (x + 1, y), (x, y), (x, y - 1), (x - 1, y)]
        .iter()
        .filter(|p| {
            p.0 == 1 && p.1 == 0
                || p.0 == width - 2 && p.1 == height - 1
                || (p.0 > 0 && p.0 < width - 1 && p.1 > 0 && p.1 < height - 1)
        })
        .map(|p| (p.0 as u8, p.1 as u8))
        .collect()
}

fn get_next_nodes(node: &Node, map: &Map) -> Vec<Node> {
    let time = node.time.checked_add(1).unwrap();
    let next_nodes = positions_to_check(&node.position, map.width, map.height)
        .iter()
        .filter(|p| map.check_position(p, time))
        .map(|p| Node { time, position: *p })
        .collect();
    next_nodes
}

fn get_next_nodes_2(node: &Node2, map: &Map, start: &Position, end: &Position) -> Vec<Node2> {
    let time = node.time.checked_add(1).unwrap();
    let next_nodes = positions_to_check(&node.position, map.width, map.height)
        .iter()
        .filter(|p| map.check_position(p, time))
        .map(|p| {
            let stage: u8 = if node.stage == 0 && p == end {
                1
            } else if node.stage == 1 && p == start {
                2
            } else {
                node.stage
            };
            Node2 {
                time,
                position: *p,
                stage,
            }
        })
        .collect();
    next_nodes
}

fn main() {
    let input = fs::read_to_string("src/day24/input.txt").unwrap();
    let map = parse_map(&input);

    let starting_position = (1u8, 0u8);
    let target_position = ((map.width - 2) as u8, (map.height - 1) as u8);
    let mut min_time = u16::MAX;

    let starting_node = Node {
        time: 0,
        position: starting_position.clone(),
    };

    let nodes = dfs_reach(starting_node, |node| {
        if node.position == target_position || node.time > 250 {
            vec![]
        } else {
            get_next_nodes(node, &map)
        }
    });

    for node in nodes {
        if node.position == target_position && node.time < min_time {
            min_time = node.time;
            println!("{}", min_time);
        }
    }

    println!("Part 1: {}", min_time);

    let mut min_time = u16::MAX;

    let starting_node = Node2 {
        time: 0,
        position: starting_position.clone(),
        stage: 0,
    };

    let nodes = dfs_reach(starting_node, |node| {
        if node.position == target_position && node.stage == 2 || node.time > 1000 {
            vec![]
        } else {
            get_next_nodes_2(node, &map, &starting_position, &target_position)
        }
    });

    for node in nodes {
        if node.position == target_position && node.stage == 2 && node.time < min_time {
            min_time = node.time;
            println!("{}", min_time);
        }
    }

    println!("Part 2: {}", min_time);
}
