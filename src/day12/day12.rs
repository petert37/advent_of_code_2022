use std::fs;

use pathfinding::{directed::astar::astar, prelude::dijkstra_all};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    fn forward_succressors(&self, elevation_map: &Vec<Vec<i32>>) -> Vec<(Pos, u32)> {
        self.successors(elevation_map, |current_height, target_height| {
            target_height - current_height <= 1
        })
    }

    fn backward_succressors(&self, elevation_map: &Vec<Vec<i32>>) -> Vec<(Pos, u32)> {
        self.successors(elevation_map, |current_height, target_height| {
            current_height - target_height <= 1
        })
    }

    fn successors<CMT>(&self, elevation_map: &Vec<Vec<i32>>, can_move_to: CMT) -> Vec<(Pos, u32)>
    where
        CMT: Fn(&i32, &i32) -> bool,
    {
        let &Pos(x, y) = self;
        let current_height = &elevation_map[y as usize][x as usize];
        [Pos(x + 1, y), Pos(x, y + 1), Pos(x - 1, y), Pos(x, y - 1)]
            .iter()
            .filter(|p| p.0 >= 0 && p.1 >= 0)
            .filter(|p| {
                if let Some(height) = elevation_map
                    .get(p.1 as usize)
                    .and_then(|row| row.get(p.0 as usize))
                {
                    can_move_to(current_height, height)
                } else {
                    false
                }
            })
            .map(|p| (p.clone(), 1))
            .collect()
    }
}

fn main() {
    let input = fs::read_to_string("src/day12/input.txt").unwrap();
    let mut elevation_map = vec![];
    let mut start_position = Pos(0, 0);
    let mut end_position = Pos(0, 0);
    input.lines().enumerate().for_each(|(y, line)| {
        let mut row = vec![];
        line.chars().enumerate().for_each(|(x, c)| {
            let elevation = match c {
                'S' => {
                    start_position = Pos(x as i32, y as i32);
                    1
                }
                'E' => {
                    end_position = Pos(x as i32, y as i32);
                    26
                }
                _ => (c as i32) - ('a' as i32) + 1,
            };
            row.push(elevation);
        });
        elevation_map.push(row);
    });

    let result = astar(
        &start_position,
        |p| p.forward_succressors(&elevation_map),
        |p| p.distance(&end_position),
        |p| *p == end_position,
    );

    if let Some((_path, length)) = result {
        println!("Path length: {length}");
    }

    let result = dijkstra_all(&end_position, |p| p.backward_succressors(&elevation_map));
    let result = result
        .iter()
        .filter(|(target, _)| {
            elevation_map
                .get(target.1 as usize)
                .and_then(|row| row.get(target.0 as usize))
                == Some(&1)
        })
        .min_by(|(_, (_, length1)), (_, (_, length2))| length1.cmp(length2));

    if let Some((_, (_, length))) = result {
        println!("Path length: {length}");
    }
}
