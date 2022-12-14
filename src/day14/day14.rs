use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Path {
    points: Vec<Point>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MapElement {
    Air,
    Rock,
    Sand,
}

fn parse_paths(input: &str) -> IResult<&str, Vec<Path>> {
    separated_list1(
        line_ending,
        separated_list1(
            tag(" -> "),
            separated_pair(complete::u32, tag(","), complete::u32).map(|(x, y)| Point { x, y }),
        )
        .map(|points| Path { points }),
    )(input)
}

fn get_map_element_at(cave_map: &mut Vec<Vec<MapElement>>, x: u32, y: u32) -> MapElement {
    let needs_extension = if let Some(row) = cave_map.get(y as usize) {
        row.len() <= x as usize
    } else {
        false
    };
    let max_y = cave_map.len() - 1;
    if needs_extension {
        for (current_y, row) in cave_map.iter_mut().enumerate() {
            let filler = if current_y == max_y {
                MapElement::Rock
            } else {
                MapElement::Air
            };
            row.resize((x + 1) as usize, filler);
        }
    }
    cave_map[y as usize][x as usize]
}

fn fill_map<D>(cave_map: &Vec<Vec<MapElement>>, is_done: D) -> i32
where
    D: Fn(&mut Vec<Vec<MapElement>>, &Point) -> bool,
{
    let mut cave_map = cave_map.clone();
    let mut falling_sand = Point { x: 500, y: 0 };
    let mut sand_count = 0;

    loop {
        if is_done(&mut cave_map, &falling_sand) {
            break;
        }

        let down = get_map_element_at(&mut cave_map, falling_sand.x, falling_sand.y + 1);
        if down == MapElement::Air {
            falling_sand = Point {
                x: falling_sand.x,
                y: falling_sand.y + 1,
            };
            continue;
        } else {
            let down_left =
                get_map_element_at(&mut cave_map, falling_sand.x - 1, falling_sand.y + 1);
            if down_left == MapElement::Air {
                falling_sand = Point {
                    x: falling_sand.x - 1,
                    y: falling_sand.y + 1,
                };
                continue;
            } else {
                let down_right =
                    get_map_element_at(&mut cave_map, falling_sand.x + 1, falling_sand.y + 1);
                if down_right == MapElement::Air {
                    falling_sand = Point {
                        x: falling_sand.x + 1,
                        y: falling_sand.y + 1,
                    };
                    continue;
                } else {
                    cave_map[falling_sand.y as usize][falling_sand.x as usize] = MapElement::Sand;
                    sand_count += 1;
                    falling_sand = Point { x: 500, y: 0 };
                }
            }
        }
    }

    sand_count
}

fn main() {
    let input = fs::read_to_string("src/day14/input.txt").unwrap();
    let (_, paths) = parse_paths(&input).unwrap();

    let x_max = paths
        .iter()
        .flat_map(|p| p.points.iter().map(|p| p.x))
        .max()
        .unwrap_or(0);
    let y_max = paths
        .iter()
        .flat_map(|p| p.points.iter().map(|p| p.y))
        .max()
        .unwrap_or(0);

    let mut cave_map = vec![vec![MapElement::Air; (x_max + 1) as usize]; (y_max + 1) as usize];
    paths.iter().for_each(|path| {
        path.points.windows(2).for_each(|point_pair| {
            let Point {
                x: start_x,
                y: start_y,
            } = point_pair[0];
            let Point { x: end_x, y: end_y } = point_pair[1];
            for y in if end_y >= start_y {
                start_y..=end_y
            } else {
                end_y..=start_y
            } {
                for x in if end_x >= start_x {
                    start_x..=end_x
                } else {
                    end_x..=start_x
                } {
                    cave_map[y as usize][x as usize] = MapElement::Rock;
                }
            }
        });
    });

    let sand_count = fill_map(&cave_map, |_, falling_sand| falling_sand.y >= y_max);
    println!("Resting sand count: {sand_count}");

    cave_map.push(vec![MapElement::Air; (x_max + 1) as usize]);
    cave_map.push(vec![MapElement::Rock; (x_max + 1) as usize]);

    let sand_count = fill_map(&cave_map, |cave_map, _| {
        get_map_element_at(cave_map, 500, 0) == MapElement::Sand
    });
    println!("Resting sand count: {sand_count}");
}
