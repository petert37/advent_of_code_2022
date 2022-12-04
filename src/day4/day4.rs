use std::{fs, ops::RangeInclusive};

struct AssignmentPair {
    first: RangeInclusive<i32>,
    second: RangeInclusive<i32>,
}

impl FromIterator<RangeInclusive<i32>> for AssignmentPair {
    fn from_iter<T: IntoIterator<Item = RangeInclusive<i32>>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        if iter.next().is_some() {
            panic!("too many elements");
        }
        AssignmentPair { first, second }
    }
}

impl AssignmentPair {
    fn fully_contains(&self) -> bool {
        (self.first.contains(self.second.start()) && self.first.contains(self.second.end()))
            || (self.second.contains(self.first.start()) && self.second.contains(self.first.end()))
    }

    fn overlaps(&self) -> bool {
        self.first.contains(self.second.start())
            || self.first.contains(self.second.end())
            || self.second.contains(self.first.start())
            || self.second.contains(self.first.end())
    }
}

fn main() {
    let assignment_pairs = fs::read_to_string("src/day4/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|range| {
                    let numbers = range
                        .split("-")
                        .map(|i| i.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    RangeInclusive::new(numbers[0], numbers[1])
                })
                .collect::<AssignmentPair>()
        })
        .collect::<Vec<AssignmentPair>>();

    let fully_containing_pairs = assignment_pairs
        .iter()
        .filter(|pair| pair.fully_contains())
        .count();
    println!("Fully containing pairs: {}", fully_containing_pairs);

    let overlapping_pairs = assignment_pairs
        .iter()
        .filter(|pair| pair.overlaps())
        .count();
    println!("Overlapping pairs: {}", overlapping_pairs);
}
