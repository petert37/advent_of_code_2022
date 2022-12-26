use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;
use nom::{branch::alt, bytes::complete::tag, multi::many1, IResult, Parser};

#[derive(Clone, Copy, Debug)]
enum RockType {
    Horizontal,
    Plus,
    Corner,
    Vertical,
    Square,
}

const ROCK_TYPES: [RockType; 5] = [
    RockType::Horizontal,
    RockType::Plus,
    RockType::Corner,
    RockType::Vertical,
    RockType::Square,
];

#[derive(Debug)]
enum Move {
    Left,
    Right,
    Down,
}

#[derive(Debug)]
struct Rock {
    positions: Vec<(i32, i32)>,
}

impl Rock {
    fn new(rock_type: &RockType, y_max: i32) -> Rock {
        let positions = match rock_type {
            RockType::Horizontal => vec![
                (2, y_max + 4),
                (3, y_max + 4),
                (4, y_max + 4),
                (5, y_max + 4),
            ],
            RockType::Plus => vec![
                (3, y_max + 6),
                (2, y_max + 5),
                (3, y_max + 5),
                (4, y_max + 5),
                (3, y_max + 4),
            ],
            RockType::Corner => vec![
                (4, y_max + 6),
                (4, y_max + 5),
                (2, y_max + 4),
                (3, y_max + 4),
                (4, y_max + 4),
            ],
            RockType::Vertical => vec![
                (2, y_max + 7),
                (2, y_max + 6),
                (2, y_max + 5),
                (2, y_max + 4),
            ],
            RockType::Square => vec![
                (2, y_max + 5),
                (3, y_max + 5),
                (2, y_max + 4),
                (3, y_max + 4),
            ],
        };
        Rock { positions }
    }

    fn get_moved_positions(&self, move_direction: &Move) -> Vec<(i32, i32)> {
        self.positions
            .iter()
            .map(|(x, y)| match move_direction {
                Move::Left => (*x - 1, *y),
                Move::Right => (*x + 1, *y),
                Move::Down => (*x, *y - 1),
            })
            .collect()
    }
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(alt((
        tag(">").map(|_| Move::Right),
        tag("<").map(|_| Move::Left),
    )))(input)
}

#[allow(dead_code)]
fn print_state(settled_rock_positions: &HashSet<(i32, i32)>, current_falling_rock: &Rock) {
    let mut lines = vec!["+-------+".to_string()];
    for y in 0..current_falling_rock
        .positions
        .iter()
        .min_by(|p1, p2| p1.1.cmp(&p2.1))
        .unwrap()
        .1
    {
        let mut line = String::new();
        line.push_str("|");
        for x in 0..7 {
            if settled_rock_positions.contains(&(x, y)) {
                line.push_str("#");
            } else {
                line.push_str(".");
            }
        }
        line.push_str("|");
        lines.push(line);
    }

    let rock_positions = current_falling_rock
        .positions
        .iter()
        .cloned()
        .collect::<HashSet<(i32, i32)>>();
    let mut rock_ys = current_falling_rock
        .positions
        .iter()
        .map(|p| p.1)
        .unique()
        .collect::<Vec<i32>>();
    rock_ys.sort();
    for y in rock_ys.iter() {
        let mut line = String::new();
        line.push_str("|");
        for x in 0..7 {
            if rock_positions.contains(&(x, *y)) {
                line.push_str("@");
            } else {
                line.push_str(".");
            }
        }
        line.push_str("|");
        lines.push(line);
    }

    lines.iter().rev().for_each(|line| {
        println!("{}", line);
    });
}

fn part_1(moves: &Vec<Move>) -> i32 {
    let mut next_rock_type = ROCK_TYPES.iter().cycle();
    let mut next_move = moves.iter().cycle();

    let mut settled_rock_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut settled_rock_count = 0;
    let mut y_max = -1;

    let mut current_falling_rock = Rock::new(next_rock_type.next().unwrap(), y_max);

    loop {
        // dbg!(&current_falling_rock);
        let next_move = next_move.next().unwrap();
        // dbg!(&next_move);
        let next_position = current_falling_rock.get_moved_positions(next_move);
        if next_position
            .iter()
            .all(|p| p.0 >= 0 && p.0 < 7 && !settled_rock_positions.contains(p))
        {
            current_falling_rock.positions = next_position;
        }
        // dbg!(&current_falling_rock);
        // dbg!(&Move::Down);
        let next_position = current_falling_rock.get_moved_positions(&Move::Down);
        if next_position
            .iter()
            .all(|p| p.1 >= 0 && !settled_rock_positions.contains(p))
        {
            current_falling_rock.positions = next_position;
        } else {
            y_max = current_falling_rock
                .positions
                .iter()
                .max_by(|p1, p2| p1.1.cmp(&p2.1))
                .unwrap()
                .1
                .max(y_max);
            let positions = current_falling_rock.positions;
            current_falling_rock = Rock::new(next_rock_type.next().unwrap(), y_max);
            settled_rock_positions.extend(positions);
            // print_state(&settled_rock_positions, &current_falling_rock);
            // println!("");
            settled_rock_count += 1;
            if settled_rock_count == 2022 {
                // print_state(&settled_rock_positions, &current_falling_rock);
                break;
            }
        }
    }

    y_max + 1
}

fn part_2(moves: &Vec<Move>) -> u64 {
    let mut next_rock_type = ROCK_TYPES.iter().cycle();
    let mut next_move = moves.iter().cycle();

    let mut settled_rock_positions: HashMap<(i32, i32), u64> = HashMap::new();
    let mut y_max = -1;
    let mut settled_rock_count: u64 = 0;

    let mut current_falling_rock = Rock::new(next_rock_type.next().unwrap(), y_max);

    loop {
        let next_move = next_move.next().unwrap();
        let next_position = current_falling_rock.get_moved_positions(next_move);
        if next_position
            .iter()
            .all(|p| p.0 >= 0 && p.0 < 7 && !settled_rock_positions.keys().contains(p))
        {
            current_falling_rock.positions = next_position;
        }
        let next_position = current_falling_rock.get_moved_positions(&Move::Down);
        if next_position
            .iter()
            .all(|p| p.1 >= 0 && !settled_rock_positions.keys().contains(p))
        {
            current_falling_rock.positions = next_position;
        } else {
            y_max = current_falling_rock
                .positions
                .iter()
                .max_by(|p1, p2| p1.1.cmp(&p2.1))
                .unwrap()
                .1
                .max(y_max);
            let positions = current_falling_rock.positions;
            current_falling_rock = Rock::new(next_rock_type.next().unwrap(), y_max);
            settled_rock_count += 1;
            for position in positions {
                settled_rock_positions.insert(position, settled_rock_count);
            }

            if settled_rock_count % 1000 == 0 {
                let settled_positions = settled_rock_positions
                    .keys()
                    .collect::<HashSet<&(i32, i32)>>();
                let mut pattern = None;

                'outer: for pattern_start in 0..(y_max - 20) {
                    let pattern_end = pattern_start + 20;
                    let scanner = settled_rock_positions
                        .keys()
                        .cloned()
                        .filter(|(_, y)| *y >= pattern_start && *y < pattern_end)
                        .collect::<Vec<(i32, i32)>>();
                    for pattern_size in pattern_end..y_max {
                        if scanner
                            .iter()
                            .all(|(x, y)| settled_positions.contains(&(*x, *y + pattern_size)))
                        {
                            let pattern_end = pattern_start + pattern_size;
                            let next_pattern_end = pattern_end + pattern_size;
                            let scanner = settled_rock_positions
                                .keys()
                                .cloned()
                                .filter(|(_, y)| *y >= pattern_start && *y < pattern_end)
                                .sorted_by(|(x1, y1), (x2, y2)| y1.cmp(&y2).then(x1.cmp(&x2)))
                                .collect::<Vec<(i32, i32)>>();
                            let scanner_2 = settled_rock_positions
                                .keys()
                                .cloned()
                                .filter(|(_, y)| *y >= pattern_end && *y < next_pattern_end)
                                .map(|(x, y)| (x, y - pattern_size))
                                .sorted_by(|(x1, y1), (x2, y2)| y1.cmp(&y2).then(x1.cmp(&x2)))
                                .collect::<Vec<(i32, i32)>>();
                            if scanner == scanner_2 {
                                pattern = Some((pattern_start, pattern_size));
                                // println!("pattern_start: {}, pattern_size: {}", pattern_start, pattern_size);
                                break 'outer;
                            }
                        }
                    }
                }

                if let Some((pattern_start, pattern_size)) = pattern {
                    // print_state(&settled_rock_positions.keys().cloned().collect(), &current_falling_rock);
                    let next_pattern_start = pattern_start + pattern_size;
                    let pattern_start_rock_count = settled_rock_positions
                        .iter()
                        .filter(|((_, y), _)| *y == pattern_start)
                        .min_by(|(_, r1), (_, r2)| r1.cmp(r2))
                        .unwrap()
                        .1;
                    let pattern_end_rock_count = settled_rock_positions
                        .iter()
                        .filter(|((_, y), _)| *y == next_pattern_start)
                        .min_by(|(_, r1), (_, r2)| r1.cmp(r2))
                        .unwrap()
                        .1;
                    let pattern_rock_count = pattern_end_rock_count - pattern_start_rock_count;

                    let target_rock_count = 1_000_000_000_000u64;
                    let num_patterns =
                        (target_rock_count - pattern_start_rock_count) / pattern_rock_count;
                    let remaining_rock_count =
                        (target_rock_count - pattern_start_rock_count) % pattern_rock_count;
                    let partial_pattern_rock_count =
                        pattern_start_rock_count + remaining_rock_count;
                    let partial_pattern_top_y: i32 = settled_rock_positions
                        .iter()
                        .filter(|(_, r)| **r == partial_pattern_rock_count)
                        .max_by(|((_, y1), _), ((_, y2), _)| y1.cmp(&y2))
                        .unwrap()
                        .0
                         .1;
                    return num_patterns * pattern_size as u64 + partial_pattern_top_y as u64 + 1;
                }
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/day17/input.txt").unwrap();
    let (_, moves) = parse_moves(&input).unwrap();

    let result = part_1(&moves);
    println!("Part 1: {}", result);

    let result = part_2(&moves);
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("src/day17/test.txt").unwrap();
        let (_, moves) = parse_moves(&input).unwrap();
        let result = part_1(&moves);
        assert_eq!(result, 3068);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("src/day17/test.txt").unwrap();
        let (_, moves) = parse_moves(&input).unwrap();
        let result = part_2(&moves);
        assert_eq!(result, 1514285714288);
    }
}
