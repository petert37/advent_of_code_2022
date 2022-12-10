use std::{collections::HashSet, fs};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct Move {
    direction: Direction,
    amount: u32,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn touching(&self, other: &Position) -> bool {
        (self.x - other.x).abs() < 2 && (self.y - other.y).abs() < 2
    }

    fn follow(&self, other: &Position) -> Position {
        if self.touching(other) {
            return self.clone();
        }
        let dx = (other.x - self.x).signum();
        let dy = (other.y - self.y).signum();
        Position {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn do_move(&self, direction: &Direction) -> Position {
        match direction {
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Up => Position {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, (direction, amount)) = separated_pair(alpha1, tag(" "), digit1)(input)?;
    let direction = match direction {
        "R" => Direction::Right,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "U" => Direction::Up,
        _ => panic!("Invalid direction: {direction}"),
    };
    let amount = amount.parse::<u32>().unwrap();
    let m = Move { direction, amount };
    Ok((input, m))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(line_ending, parse_move)(input)
}

fn visit(knots: Vec<Position>, moves: &Vec<Move>) -> usize {
    if knots.is_empty() {
        return 0;
    }

    let mut knots = knots;
    let mut visited_positions = HashSet::<Position>::new();
    visited_positions.insert(knots.last().unwrap().clone());

    moves.iter().for_each(|m| {
        (0..m.amount).for_each(|_| {
            let mut moved_knots = vec![];
            moved_knots.push(knots.first().unwrap().do_move(&m.direction));

            for i in 1..knots.len() {
                moved_knots.push(knots[i].follow(moved_knots.last().unwrap()));
            }

            visited_positions.insert(moved_knots.last().unwrap().clone());
            knots = moved_knots;
        });
    });

    visited_positions.len()
}

fn main() {
    let input = fs::read_to_string("src/day9/input.txt").unwrap();
    let (_, moves) = parse_moves(&input).unwrap();

    let short_rope = vec![Position { x: 0, y: 0 }; 2];
    let short_rope_visited_count = visit(short_rope, &moves);
    println!("Visited positions: {}", short_rope_visited_count);

    let long_rope = vec![Position { x: 0, y: 0 }; 10];
    let long_rope_visited_count = visit(long_rope, &moves);
    println!("Visited positions: {}", long_rope_visited_count);
}
