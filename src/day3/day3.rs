use std::{collections::HashSet, fs, hash::Hash};

struct Rucksack {
    compartment1: Vec<char>,
    compartment2: Vec<char>,
}

impl Rucksack {
    fn from_line(line: &str) -> Rucksack {
        let chars = line.chars().collect::<Vec<char>>();
        let size = chars.len();
        Rucksack {
            compartment1: chars.get(0..(size / 2)).unwrap().into(),
            compartment2: chars.get((size / 2)..size).unwrap().into(),
        }
    }

    fn item_types_in_both_compartments(&self) -> Vec<char> {
        vector_intersection(&self.compartment1, &self.compartment2)
    }
}

fn get_item_priority(item: &char) -> i32 {
    if item.is_uppercase() {
        (*item as i32) - ('A' as i32) + 27
    } else {
        (*item as i32) - ('a' as i32) + 1
    }
}

fn vector_intersection<T: Eq + Hash + Clone>(vec1: &Vec<T>, vec2: &Vec<T>) -> Vec<T> {
    let c1: HashSet<T> = HashSet::from_iter(vec1.iter().cloned());
    let c2: HashSet<T> = HashSet::from_iter(vec2.iter().cloned());
    c1.intersection(&c2).cloned().collect()
}

fn main() {
    let rucksacks = fs::read_to_string("src/day3/input.txt")
        .unwrap()
        .lines()
        .map(Rucksack::from_line)
        .collect::<Vec<Rucksack>>();

    let sum_priorities = rucksacks
        .iter()
        .map(|rucksack| {
            rucksack
                .item_types_in_both_compartments()
                .iter()
                .map(get_item_priority)
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("Sum priorities: {}", sum_priorities);

    let sum_badge_priorities = rucksacks
        .chunks(3)
        .map(|group| {
            group
                .iter()
                .map(|rucksack| {
                    rucksack
                        .compartment1
                        .iter()
                        .cloned()
                        .chain(rucksack.compartment2.iter().cloned())
                        .collect::<Vec<char>>()
                })
                .reduce(|acc, item| vector_intersection(&acc, &item))
                .unwrap()
                .iter()
                .map(get_item_priority)
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("Sum badge priorities: {}", sum_badge_priorities);
}
