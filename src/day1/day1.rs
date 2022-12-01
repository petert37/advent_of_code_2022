use std::fs;

fn main() {
    let mut sum_calories = fs::read_to_string("src/day1/input.txt")
        .unwrap()
        .replace("\r", "")
        .split("\n\n")
        .map(|elf_calories| {
            elf_calories
                .split("\n")
                .map(|cal| cal.parse::<i32>().unwrap())
                .sum()
        })
        .collect::<Vec<i32>>();
    sum_calories.sort();
    sum_calories.reverse();

    if let Some(max_sum_calories) = sum_calories.first() {
        println!("Max sum calories: {}", max_sum_calories);
    };

    let top_three_sum_calories = sum_calories[0..3].iter().sum::<i32>();
    println!("Top three sum calories: {}", top_three_sum_calories);
}
