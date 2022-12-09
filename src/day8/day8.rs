use std::{collections::HashSet, fs};

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
struct Tree {
    x: i32,
    y: i32,
    size: i32,
}

fn column(forest: &Vec<Vec<Tree>>, index: usize) -> Option<Vec<Tree>> {
    let mut result: Vec<Tree> = vec![];
    for row in forest {
        if let Some(c) = row.get(index) {
            result.push(c.clone());
        } else {
            return None;
        }
    }
    return Some(result);
}

fn get_visible_trees<'a>(trees: impl Iterator<Item = &'a Tree>) -> HashSet<Tree> {
    let mut visible_trees = HashSet::new();
    let mut max_height: Option<i32> = None;
    for tree in trees {
        match max_height {
            Some(mh) => {
                if tree.size > mh {
                    max_height = Some(tree.size);
                    visible_trees.insert(tree.clone());
                }
            }
            None => {
                max_height = Some(tree.size);
                visible_trees.insert(tree.clone());
            }
        }
    }
    return visible_trees;
}

fn get_tree<'a>(forest: &'a Vec<Vec<Tree>>, x: usize, y: usize) -> Option<&'a Tree> {
    forest.get(y).and_then(|row| row.get(x))
}

fn main() {
    let input = fs::read_to_string("src/day8/input.txt").unwrap();
    let forest = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Tree {
                    x: x as i32,
                    y: y as i32,
                    size: c.to_digit(10).unwrap() as i32,
                })
                .collect::<Vec<Tree>>()
        })
        .collect::<Vec<Vec<Tree>>>();

    let row_count = forest.len();
    let column_count = match forest.first() {
        Some(f) => f.len(),
        None => 0,
    };

    let mut visible_trees = HashSet::new();

    for row in forest.iter() {
        visible_trees.extend(get_visible_trees(row.iter()));
        visible_trees.extend(get_visible_trees(row.iter().rev()));
    }

    for ci in 0..column_count {
        if let Some(column) = &column(&forest, ci) {
            visible_trees.extend(get_visible_trees(column.iter()));
            visible_trees.extend(get_visible_trees(column.iter().rev()));
        }
    }

    println!("Visible tree count: {}", visible_trees.len());

    let mut max_scenic_score = -1;

    for y in 1..row_count - 1 {
        for x in 1..column_count - 1 {
            if let Some(tree) = get_tree(&forest, x, y) {
                
                let mut dist_right = 0;
                for dx in x + 1..column_count {
                    if let Some(other_tree) = get_tree(&forest, dx, y) {
                        dist_right += 1;
                        if other_tree.size >= tree.size {
                            break;
                        }
                    }
                }

                let mut dist_down = 0;
                for dy in y + 1..row_count {
                    if let Some(other_tree) = get_tree(&forest, x, dy) {
                        dist_down += 1;
                        if other_tree.size >= tree.size {
                            break;
                        }
                    }
                }

                let mut dist_left = 0;
                for dx in (0..x).rev() {
                    if let Some(other_tree) = get_tree(&forest, dx, y) {
                        dist_left += 1;
                        if other_tree.size >= tree.size {
                            break;
                        }
                    }
                }

                let mut dist_up = 0;
                for dy in (0..y).rev() {
                    if let Some(other_tree) = get_tree(&forest, x, dy) {
                        dist_up += 1;
                        if other_tree.size >= tree.size {
                            break;
                        }
                    }
                }

                let scenic_score = dist_right * dist_down * dist_left * dist_up;

                if scenic_score > max_scenic_score {
                    max_scenic_score = scenic_score;
                }
            }
        }
    }

    println!("Highest scenic score: {}", max_scenic_score);
}
