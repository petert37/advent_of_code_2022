use std::fs;

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Instruction::Noop))
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, value)) = separated_pair(tag("addx"), tag(" "), complete::i32)(input)?;
    Ok((input, Instruction::AddX(value)))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list1(line_ending, alt((parse_noop, parse_addx)))(input)?;
    Ok((input, instructions))
}

enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn get_num_cycles(&self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

fn print_screen(screen: &Vec<Vec<&str>>) {
    screen.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{c}"));
        println!("");
    });
}

fn get_sprite(x: &i32) -> [i32; 3] {
    [x - 1, *x, x + 1]
}

fn get_cycle_coords(cycle: &i32) -> (i32, i32) {
    let y = (cycle - 1) / 40;
    let x = cycle - y * 40 - 1;
    (x, y)
}

fn main() {
    let input = fs::read_to_string("src/day10/input.txt").unwrap();
    let (_, instructions) = parse_instructions(&input).unwrap();

    let interesting_cycles = [20, 60, 100, 140, 180, 220];
    let mut sum_signal_strength = 0;

    let mut screen = vec![vec!["."; 40]; 6];

    let mut cycle: i32 = 1;
    let mut pc: usize = 0;
    let mut x: i32 = 1;
    let mut current_instruction_remaining_cycles = 0;

    while pc < instructions.len() {
        let instruction = &instructions[pc];
        if current_instruction_remaining_cycles == 0 {
            current_instruction_remaining_cycles = instruction.get_num_cycles();
        }

        if interesting_cycles.contains(&cycle) {
            sum_signal_strength += cycle * x;
        }

        let sprite = get_sprite(&x);
        let coords = get_cycle_coords(&cycle);
        if sprite.contains(&coords.0) {
            screen[coords.1 as usize][coords.0 as usize] = "#";
        }

        current_instruction_remaining_cycles -= 1;

        if current_instruction_remaining_cycles == 0 {
            match instruction {
                Instruction::Noop => {}
                Instruction::AddX(value) => x += value,
            }

            pc += 1;
        }
        cycle += 1;
    }

    println!("Sum signal strength: {sum_signal_strength}");

    print_screen(&screen);
}
