use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn directions_to_check(&self, direction: &Direction) -> [Position; 3] {
        match direction {
            Direction::North => [
                Position {
                    x: self.x - 1,
                    y: self.y - 1,
                },
                Position {
                    x: self.x,
                    y: self.y - 1,
                },
                Position {
                    x: self.x + 1,
                    y: self.y - 1,
                },
            ],
            Direction::South => [
                Position {
                    x: self.x - 1,
                    y: self.y + 1,
                },
                Position {
                    x: self.x,
                    y: self.y + 1,
                },
                Position {
                    x: self.x + 1,
                    y: self.y + 1,
                },
            ],
            Direction::West => [
                Position {
                    x: self.x - 1,
                    y: self.y - 1,
                },
                Position {
                    x: self.x - 1,
                    y: self.y,
                },
                Position {
                    x: self.x - 1,
                    y: self.y + 1,
                },
            ],
            Direction::East => [
                Position {
                    x: self.x + 1,
                    y: self.y - 1,
                },
                Position {
                    x: self.x + 1,
                    y: self.y,
                },
                Position {
                    x: self.x + 1,
                    y: self.y + 1,
                },
            ],
        }
    }

    fn move_tovards(&self, direction: &Direction) -> Position {
        self.directions_to_check(&direction)[1]
    }

    fn get_surrounding_positions(&self) -> [Position; 8] {
        [
            Position {
                x: self.x - 1,
                y: self.y - 1,
            },
            Position {
                x: self.x,
                y: self.y - 1,
            },
            Position {
                x: self.x + 1,
                y: self.y - 1,
            },
            Position {
                x: self.x + 1,
                y: self.y,
            },
            Position {
                x: self.x + 1,
                y: self.y + 1,
            },
            Position {
                x: self.x,
                y: self.y + 1,
            },
            Position {
                x: self.x - 1,
                y: self.y + 1,
            },
            Position {
                x: self.x - 1,
                y: self.y,
            },
        ]
    }
}

enum Direction {
    North,
    South,
    West,
    East,
}

fn parse_positions(input: &str) -> HashSet<Position> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .map(move |(x, _)| Position {
                    x: x as i32,
                    y: y as i32,
                })
        })
        .collect::<HashSet<Position>>()
}

fn get_bounding_box(positions: &HashSet<Position>) -> [Position; 2] {
    let min_x = positions.iter().min_by(|p1, p2| p1.x.cmp(&p2.x)).unwrap().x;
    let max_x = positions.iter().max_by(|p1, p2| p1.x.cmp(&p2.x)).unwrap().x;
    let min_y = positions.iter().min_by(|p1, p2| p1.y.cmp(&p2.y)).unwrap().y;
    let max_y = positions.iter().max_by(|p1, p2| p1.y.cmp(&p2.y)).unwrap().y;
    [
        Position { x: min_x, y: min_y },
        Position { x: max_x, y: max_y },
    ]
}

#[allow(dead_code)]
fn print_positions(positions: &HashSet<Position>) {
    let [min, max] = get_bounding_box(positions);
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let c = if positions.contains(&Position { x, y }) {
                "#"
            } else {
                "."
            };
            print!("{}", c);
        }
        println!("");
    }
    println!("");
}

fn simulate(positions: &mut HashSet<Position>, part_2: bool) -> i32 {
    let mut directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]
    .iter()
    .cycle();
    // print_positions(&positions);
    let mut loops = 0;
    loop {
        let (first, second, third, fourth) = directions.next_tuple().unwrap();
        let directions_to_check = [first, second, third, fourth];
        let mut new_positions: HashMap<Position, Vec<Position>> = HashMap::new();

        for current_position in &*positions {
            let mut new_position = *current_position;
            if current_position
                .get_surrounding_positions()
                .iter()
                .any(|p| positions.get(p).is_some())
            {
                for direction_to_check in directions_to_check.iter() {
                    if current_position
                        .directions_to_check(&direction_to_check)
                        .iter()
                        .all(|p| positions.get(p).is_none())
                    {
                        new_position = current_position.move_tovards(&direction_to_check);
                        break;
                    }
                }
            }
            new_positions
                .entry(new_position)
                .or_insert(vec![])
                .push(*current_position);
        }

        let mut moved_positions = HashSet::new();
        for (new_position, old_positions) in new_positions {
            if old_positions.len() == 1 {
                moved_positions.insert(new_position);
            } else {
                moved_positions.extend(old_positions);
            }
        }

        directions.next();
        loops += 1;

        if part_2 {
            if *positions == moved_positions {
                return loops;
            }
            *positions = moved_positions;
        } else {
            *positions = moved_positions;
            // print_positions(&positions);
            if loops == 10 {
                return loops;
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/day23/input.txt").unwrap();
    let positions = parse_positions(&input);

    let mut positions_1 = positions.clone();
    simulate(&mut positions_1, false);

    let [min, max] = get_bounding_box(&positions_1);
    let size = (max.x - min.x + 1) * (max.y - min.y + 1);
    let elf_count = positions_1.len() as i32;
    let empty_ground_tiles = size - elf_count;

    println!("Empty ground tiles: {}", empty_ground_tiles);

    let mut positions_2 = positions.clone();
    let loops = simulate(&mut positions_2, true);

    println!("Total rounds: {}", loops);
}
