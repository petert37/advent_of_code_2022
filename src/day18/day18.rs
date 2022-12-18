use std::{collections::HashSet, fs, thread};

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};

const NEIGHBORS: [(i32, i32, i32); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn parse_droplet(input: &str) -> IResult<&str, HashSet<(i32, i32, i32)>> {
    separated_list1(
        line_ending,
        tuple((
            complete::i32,
            tag(","),
            complete::i32,
            tag(","),
            complete::i32,
        ))
        .map(|(x, _, y, _, z)| (x, y, z)),
    )
    .map(|coords| coords.into_iter().collect())
    .parse(input)
}

fn fill(cube: &mut HashSet<(i32, i32, i32)>, position: (i32, i32, i32), min: &i32, max: &i32) {
    if position.0 < *min
        || position.0 > *max
        || position.1 < *min
        || position.1 > *max
        || position.2 < *min
        || position.2 > *max
    {
        return;
    }
    if cube.contains(&position) {
        return;
    }
    cube.insert(position);
    for neighbor in NEIGHBORS {
        fill(
            cube,
            (
                position.0 + neighbor.0,
                position.1 + neighbor.1,
                position.2 + neighbor.2,
            ),
            min,
            max,
        );
    }
}

fn count_sides(droplet: &HashSet<(i32, i32, i32)>, to_check: &HashSet<(i32, i32, i32)>) -> i32 {
    let sides = droplet
        .iter()
        .map(|cube| {
            NEIGHBORS
                .iter()
                .filter(|&neighbor| {
                    !to_check.contains(&(
                        cube.0 + neighbor.0,
                        cube.1 + neighbor.1,
                        cube.2 + neighbor.2,
                    ))
                })
                .count() as i32
        })
        .sum::<i32>();
    sides
}

fn main() {
    let input = fs::read_to_string("src/day18/input.txt").unwrap();
    let (_, droplet) = parse_droplet(&input).unwrap();

    let sides = count_sides(&droplet, &droplet);

    println!("Total sides: {}", sides);

    let mut cube = droplet.clone();

    //Needs a larger stack size because of recursion in fill
    let inner_sides = thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(move || {
            fill(&mut cube, (-1, -1, -1), &-1, &20);
            count_sides(&droplet, &cube)
        })
        .unwrap()
        .join()
        .unwrap();

    println!("Outer sides: {}", sides - inner_sides);
}
