use std::fs;

use itertools::Itertools;

fn calc_new_index(size: usize, i: usize, di: i64) -> usize {
    let size = size as i64;
    let mut new_index = (i as i64 + di) % (size - 1);
    if new_index < 0 {
        new_index = new_index + size - 1;
    }
    new_index as usize
}

fn mix(numbers: &mut Vec<(usize, i64)>) {
    let size = numbers.len();
    for i in 0..size {
        if let Some((index, _)) = numbers
            .iter()
            .find_position(|(original_index, _)| *original_index == i)
        {
            let removed = numbers.remove(index);
            let new_index = calc_new_index(size, index, removed.1);
            numbers.insert(new_index, removed);
        }
    }
}

fn calc_result(numbers: &Vec<(usize, i64)>) -> i64 {
    let count = numbers.len();
    let (zero_index, _) = numbers.iter().find_position(|(_, num)| *num == 0).unwrap();
    numbers[(zero_index + 1000) % count].1
        + numbers[(zero_index + 2000) % count].1
        + numbers[(zero_index + 3000) % count].1
}

fn main() {
    let input = fs::read_to_string("src/day20/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut numbers = input
        .iter()
        .cloned()
        .enumerate()
        .collect::<Vec<(usize, i64)>>();
    mix(&mut numbers);
    let result = calc_result(&numbers);
    println!("Part 1: {}", result);

    let mut numbers = input
        .iter()
        .cloned()
        .map(|num| num * 811589153)
        .enumerate()
        .collect::<Vec<(usize, i64)>>();
    (0..10).for_each(|_| mix(&mut numbers));
    let result = calc_result(&numbers);
    println!("Part 2: {}", result);
}
