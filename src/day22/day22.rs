use std::{collections::BTreeMap, fs, ops::Range};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::many1,
    sequence::terminated,
    IResult, Parser,
};

#[derive(Debug, PartialEq)]
enum Tile {
    Void,
    Space,
    Wall,
}

impl Tile {
    fn get_char(&self) -> char {
        match self {
            Tile::Void => ' ',
            Tile::Space => '.',
            Tile::Wall => '#',
        }
    }
}

#[derive(Debug)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    actions: Vec<Action>,
}

impl Board {
    fn get_starting_position(&self) -> Position {
        let x = self.tiles[0]
            .iter()
            .find_position(|&tile| tile == &Tile::Space)
            .unwrap()
            .0 as i32;
        let y = 0;
        Position {
            x,
            y,
            facing: Direction::Right,
        }
    }

    fn perform_action(&self, action: &Action, position: &mut Position) {
        match action {
            Action::Move(amount) => {
                for _ in 0..*amount {
                    let (new_x, new_y) = match position.facing {
                        Direction::Up => (position.x, position.y - 1),
                        Direction::Right => (position.x + 1, position.y),
                        Direction::Down => (position.x, position.y + 1),
                        Direction::Left => (position.x - 1, position.y),
                    };
                    match self
                        .tiles
                        .get(new_y as usize)
                        .and_then(|row| row.get(new_x as usize))
                    {
                        None | Some(Tile::Void) => match position.facing {
                            Direction::Right => {
                                let new_x = self
                                    .tiles
                                    .get(new_y as usize)
                                    .and_then(|row| {
                                        row.iter()
                                            .enumerate()
                                            .find(|(_, tile)| tile != &&Tile::Void)
                                    })
                                    .unwrap();
                                if new_x.1 == &Tile::Space {
                                    position.x = new_x.0 as i32;
                                }
                            }
                            Direction::Left => {
                                let new_x = self
                                    .tiles
                                    .get(new_y as usize)
                                    .and_then(|row| {
                                        row.iter()
                                            .enumerate()
                                            .rev()
                                            .find(|(_, tile)| tile != &&Tile::Void)
                                    })
                                    .unwrap();
                                if new_x.1 == &Tile::Space {
                                    position.x = new_x.0 as i32;
                                }
                            }
                            Direction::Down => {
                                let new_y = self
                                    .tiles
                                    .iter()
                                    .enumerate()
                                    .find(|(_, tiles)| {
                                        let tile = tiles.get(new_x as usize);
                                        tile.is_some() && tile.unwrap() != &Tile::Void
                                    })
                                    .unwrap();
                                if new_y.1.get(new_x as usize) == Some(&Tile::Space) {
                                    position.y = new_y.0 as i32;
                                }
                            }
                            Direction::Up => {
                                let new_y = self
                                    .tiles
                                    .iter()
                                    .enumerate()
                                    .rev()
                                    .find(|(_, tiles)| {
                                        let tile = tiles.get(new_x as usize);
                                        tile.is_some() && tile.unwrap() != &Tile::Void
                                    })
                                    .unwrap();
                                if new_y.1.get(new_x as usize) == Some(&Tile::Space) {
                                    position.y = new_y.0 as i32;
                                }
                            }
                        },
                        Some(Tile::Wall) => {}
                        Some(Tile::Space) => {
                            position.x = new_x;
                            position.y = new_y;
                        }
                    }
                }
            }
            Action::Turn(direction) => position.facing = position.facing.turn(direction),
        }
    }

    fn perform_action_cube(
        &self,
        action: &Action,
        position: &mut Position,
        cube_faces: &BTreeMap<i32, CubeFace>,
    ) {
        match action {
            Action::Move(amount) => {
                for _ in 0..*amount {
                    let (new_x, new_y) = match position.facing {
                        Direction::Up => (position.x, position.y - 1),
                        Direction::Right => (position.x + 1, position.y),
                        Direction::Down => (position.x, position.y + 1),
                        Direction::Left => (position.x - 1, position.y),
                    };
                    match self
                        .tiles
                        .get(new_y as usize)
                        .and_then(|row| row.get(new_x as usize))
                    {
                        None | Some(Tile::Void) => {
                            let current_face = cube_faces
                                .values()
                                .find(|face| {
                                    face.x_range.contains(&position.x)
                                        && face.y_range.contains(&position.y)
                                })
                                .unwrap();

                            let (new_x, new_y, new_facing) = match position.facing {
                                Direction::Right => {
                                    let new_face =
                                        cube_faces.get(&current_face.right.unwrap()).unwrap();
                                    if new_face.up.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.start
                                                + (current_face.y_range.end - 1 - position.y),
                                            new_face.y_range.start,
                                            Direction::Down,
                                        )
                                    } else if new_face.right.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.end - 1,
                                            new_face.y_range.start
                                                + (current_face.y_range.end - 1 - position.y),
                                            Direction::Left,
                                        )
                                    } else if new_face.down.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.start
                                                + (position.y - current_face.y_range.start),
                                            new_face.y_range.end - 1,
                                            Direction::Up,
                                        )
                                    } else if new_face.left.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.start,
                                            new_face.y_range.start
                                                + (position.y - current_face.y_range.start),
                                            Direction::Right,
                                        )
                                    } else {
                                        panic!("!")
                                    }
                                }
                                Direction::Left => {
                                    let new_face =
                                        cube_faces.get(&current_face.left.unwrap()).unwrap();
                                    if new_face.up.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.start
                                                + (position.y - current_face.y_range.start),
                                            new_face.y_range.start,
                                            Direction::Down,
                                        )
                                    } else if new_face.right.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.end - 1,
                                            new_face.y_range.start
                                                + (position.y - current_face.y_range.start),
                                            Direction::Left,
                                        )
                                    } else if new_face.down.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.start
                                                + (current_face.y_range.end - 1 - position.y),
                                            new_face.y_range.end - 1,
                                            Direction::Up,
                                        )
                                    } else if new_face.left.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.start,
                                            new_face.y_range.start
                                                + (current_face.y_range.end - 1 - position.y),
                                            Direction::Right,
                                        )
                                    } else {
                                        panic!("!")
                                    }
                                }
                                Direction::Down => {
                                    let new_face =
                                        cube_faces.get(&current_face.down.unwrap()).unwrap();
                                    if new_face.up.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.start
                                                + (position.x - current_face.x_range.start),
                                            new_face.y_range.start,
                                            Direction::Down,
                                        )
                                    } else if new_face.right.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.end - 1,
                                            new_face.y_range.start
                                                + (position.x - current_face.x_range.start),
                                            Direction::Left,
                                        )
                                    } else if new_face.down.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.start
                                                + (current_face.x_range.end - 1 - position.x),
                                            new_face.y_range.end - 1,
                                            Direction::Up,
                                        )
                                    } else if new_face.left.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.start,
                                            new_face.y_range.start
                                                + (current_face.x_range.end - 1 - position.x),
                                            Direction::Right,
                                        )
                                    } else {
                                        panic!("!")
                                    }
                                }
                                Direction::Up => {
                                    let new_face =
                                        cube_faces.get(&current_face.up.unwrap()).unwrap();
                                    if new_face.up.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.start
                                                + (current_face.x_range.end - 1 - position.x),
                                            new_face.y_range.start,
                                            Direction::Down,
                                        )
                                    } else if new_face.right.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.end - 1,
                                            new_face.y_range.start
                                                + (current_face.x_range.end - 1 - position.x),
                                            Direction::Left,
                                        )
                                    } else if new_face.down.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.start
                                                + (position.x - current_face.x_range.start),
                                            new_face.y_range.end - 1,
                                            Direction::Up,
                                        )
                                    } else if new_face.left.unwrap() == current_face.face {
                                        (
                                            new_face.x_range.start,
                                            new_face.y_range.start
                                                + (position.x - current_face.x_range.start),
                                            Direction::Right,
                                        )
                                    } else {
                                        panic!("!")
                                    }
                                }
                            };

                            if let Some(Tile::Space) = self
                                .tiles
                                .get(new_y as usize)
                                .and_then(|row| row.get(new_x as usize))
                            {
                                position.x = new_x;
                                position.y = new_y;
                                position.facing = new_facing;
                            }
                        }
                        Some(Tile::Wall) => {}
                        Some(Tile::Space) => {
                            position.x = new_x;
                            position.y = new_y;
                        }
                    }
                }
            }
            Action::Turn(direction) => position.facing = position.facing.turn(direction),
        }
    }
}

#[derive(Debug)]
enum Action {
    Move(u32),
    Turn(Direction),
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self, direction: &Direction) -> Direction {
        match direction {
            Direction::Right => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            Direction::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            },
            _ => panic!("Invalid turn"),
        }
    }

    fn get_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    facing: Direction,
}

impl Position {
    fn get_score(&self) -> i32 {
        let facing_score = match self.facing {
            Direction::Up => 3,
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
        };

        1000 * (self.y + 1) + 4 * (self.x + 1) + facing_score
    }
}

#[derive(Debug)]
struct CubeFace {
    face: i32,
    up: Option<i32>,
    right: Option<i32>,
    down: Option<i32>,
    left: Option<i32>,
    x_range: Range<i32>,
    y_range: Range<i32>,
}

fn parse_board(input: &str) -> IResult<&str, Board> {
    let (input, tiles) = many1(terminated(
        many1(alt((
            tag(" ").map(|_| Tile::Void),
            tag(".").map(|_| Tile::Space),
            tag("#").map(|_| Tile::Wall),
        ))),
        line_ending,
    ))(input)?;
    let (input, _) = line_ending(input)?;
    let (input, actions) = many1(alt((
        complete::u32.map(|num| Action::Move(num)),
        tag("L").map(|_| Action::Turn(Direction::Left)),
        tag("R").map(|_| Action::Turn(Direction::Right)),
    )))(input)?;
    Ok((input, Board { tiles, actions }))
}

fn print_state(board: &Board, position: &Position) {
    board.tiles.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, tile)| {
            if position.x == x as i32 && position.y == y as i32 {
                print!("{}", position.facing.get_char());
            } else {
                print!("{}", tile.get_char());
            }
        });
        println!("");
    });
}

fn rotate_left(other_face: &CubeFace, face_id: i32) -> Option<i32> {
    if other_face.up == Some(face_id) {
        other_face.left
    } else if other_face.right == Some(face_id) {
        other_face.up
    } else if other_face.down == Some(face_id) {
        other_face.right
    } else if other_face.left == Some(face_id) {
        other_face.down
    } else {
        None
    }
}

fn rotate_right(other_face: &CubeFace, face_id: i32) -> Option<i32> {
    if other_face.up == Some(face_id) {
        other_face.right
    } else if other_face.right == Some(face_id) {
        other_face.down
    } else if other_face.down == Some(face_id) {
        other_face.left
    } else if other_face.left == Some(face_id) {
        other_face.up
    } else {
        None
    }
}

fn make_cube_faces(board: &Board, face_width: i32, face_height: i32) -> BTreeMap<i32, CubeFace> {
    let board_width = board.tiles.iter().map(|row| row.len()).max().unwrap_or(0) as i32;
    let board_height = board.tiles.len() as i32;
    let mut faces: BTreeMap<i32, CubeFace> = BTreeMap::new();
    let mut face_id = 1;

    for face_y in 0..(board_height / face_height) {
        for face_x in 0..(board_width / face_width) {
            if let Some(Tile::Space) | Some(Tile::Wall) = board
                .tiles
                .get((face_height * face_y) as usize)
                .and_then(|row| row.get((face_width * face_x) as usize))
            {
                let cube_face = CubeFace {
                    face: face_id,
                    up: None,
                    right: None,
                    down: None,
                    left: None,
                    x_range: (face_width * face_x)..(face_width * (face_x + 1)),
                    y_range: (face_height * face_y)..(face_height * (face_y + 1)),
                };
                faces.insert(face_id, cube_face);
                face_id += 1;
            }
        }
    }

    let face_ids = faces.keys().cloned().collect::<Vec<i32>>();

    for face_id in face_ids.iter() {
        let face = faces.get(face_id).unwrap();

        let up = face.up.or_else(|| {
            faces
                .values()
                .find(|other_face| {
                    other_face.x_range == face.x_range
                        && other_face.y_range.end == face.y_range.start
                })
                .and_then(|other_face| Some(other_face.face))
        });
        let right = face.right.or_else(|| {
            faces
                .values()
                .find(|other_face| {
                    other_face.y_range == face.y_range
                        && other_face.x_range.start == face.x_range.end
                })
                .and_then(|other_face| Some(other_face.face))
        });
        let down = face.down.or_else(|| {
            faces
                .values()
                .find(|other_face| {
                    other_face.x_range == face.x_range
                        && other_face.y_range.start == face.y_range.end
                })
                .and_then(|other_face| Some(other_face.face))
        });
        let left = face.left.or_else(|| {
            faces
                .values()
                .find(|other_face| {
                    other_face.y_range == face.y_range
                        && other_face.x_range.end == face.x_range.start
                })
                .and_then(|other_face| Some(other_face.face))
        });

        let face = faces.get_mut(face_id).unwrap();
        face.up = up;
        face.right = right;
        face.down = down;
        face.left = left;
    }

    while faces.values().any(|face| {
        face.up.is_none() || face.right.is_none() || face.down.is_none() || face.left.is_none()
    }) {
        for &face_id in face_ids.iter() {
            let face = faces.get(&face_id).unwrap();

            let up = face.up.or_else(|| {
                face.right
                    .and_then(|right| {
                        faces
                            .get(&right)
                            .and_then(|right_face| rotate_right(right_face, face_id))
                    })
                    .or_else(|| {
                        face.left.and_then(|left| {
                            faces
                                .get(&left)
                                .and_then(|left_face| rotate_left(left_face, face_id))
                        })
                    })
            });

            let right = face.right.or_else(|| {
                face.up
                    .and_then(|up| {
                        faces
                            .get(&up)
                            .and_then(|up_face| rotate_left(up_face, face_id))
                    })
                    .or_else(|| {
                        face.down.and_then(|down| {
                            faces
                                .get(&down)
                                .and_then(|down_face| rotate_right(down_face, face_id))
                        })
                    })
            });

            let down = face.down.or_else(|| {
                face.right
                    .and_then(|right| {
                        faces
                            .get(&right)
                            .and_then(|right_face| rotate_left(right_face, face_id))
                    })
                    .or_else(|| {
                        face.left.and_then(|left| {
                            faces
                                .get(&left)
                                .and_then(|left_face| rotate_right(left_face, face_id))
                        })
                    })
            });

            let left = face.left.or_else(|| {
                face.up
                    .and_then(|up| {
                        faces
                            .get(&up)
                            .and_then(|up_face| rotate_right(up_face, face_id))
                    })
                    .or_else(|| {
                        face.down.and_then(|down| {
                            faces
                                .get(&down)
                                .and_then(|down_face| rotate_left(down_face, face_id))
                        })
                    })
            });

            let face = faces.get_mut(&face_id).unwrap();
            face.up = up;
            face.right = right;
            face.down = down;
            face.left = left;
        }
    }

    faces
}

fn part_1(board: &Board) -> i32 {
    let mut position = board.get_starting_position();
    // print_state(&board, &position);
    for action in &board.actions {
        //dbg!(&action);
        board.perform_action(action, &mut position);
        // print_state(&board, &position);
    }

    position.get_score()
}

fn part_2(board: &Board, face_width: i32, face_height: i32) -> i32 {
    let cube_faces = make_cube_faces(&board, face_width, face_height);
    // dbg!(&cube_faces);
    let mut position = board.get_starting_position();
    // print_state(&board, &position);
    for action in &board.actions {
        // dbg!(&action);
        board.perform_action_cube(action, &mut position, &cube_faces);
        // print_state(&board, &position);
    }

    position.get_score()
}

fn main() {
    let input = fs::read_to_string("src/day22/input.txt").unwrap();
    let (_, board) = parse_board(&input).unwrap();

    let score = part_1(&board);
    println!("Part 1: {}", score);

    let score = part_2(&board, 50, 50);
    println!("Part 2: {}", score);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("src/day22/test.txt").unwrap();
        let (_, board) = parse_board(&input).unwrap();
        let result = part_1(&board);
        assert_eq!(result, 6032)
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("src/day22/test.txt").unwrap();
        let (_, board) = parse_board(&input).unwrap();
        let result = part_2(&board, 4, 4);
        assert_eq!(result, 5031)
    }
}
