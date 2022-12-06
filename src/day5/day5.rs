use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, digit1, line_ending, not_line_ending},
    multi::{count, many1, separated_list1},
    sequence::{delimited, terminated},
    IResult,
};

#[derive(Debug)]
struct Move {
    from: i32,
    to: i32,
    amount: i32,
}

fn parse_actual_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, chr) = delimited(tag("["), alpha1, tag("]"))(input)?;
    let (_, chr) = anychar(chr)?;
    Ok((input, Some(chr)))
}

fn parse_missing_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, None))
}

fn parse_one_crate(input: &str) -> IResult<&str, Option<char>> {
    alt((parse_actual_crate, parse_missing_crate))(input)
}

fn parse_crate_row(input: &str) -> IResult<&str, Vec<Option<char>>> {
    terminated(separated_list1(tag(" "), parse_one_crate), line_ending)(input)
}

fn parse_crate_rows(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    many1(parse_crate_row)(input)
}

fn parse_move_line(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, amount) = digit1(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = digit1(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = digit1(input)?;
    Ok((
        input,
        Move {
            from: from.parse::<i32>().unwrap(),
            to: to.parse::<i32>().unwrap(),
            amount: amount.parse::<i32>().unwrap(),
        },
    ))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(line_ending, parse_move_line)(input)
}

fn skip_line(input: &str) -> IResult<&str, Option<&str>> {
    let (input, _) = not_line_ending(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, None))
}

fn get_top_row(piles: &Vec<Vec<char>>) -> String {
    piles
        .iter()
        .map(|pile| match pile.last() {
            Some(c) => *c,
            None => ' ',
        })
        .collect::<String>()
}

fn main() {
    let input = fs::read_to_string("src/day5/input.txt").unwrap();
    let (input, crates) = parse_crate_rows(&input).unwrap();
    let (input, _) = count(skip_line, 2)(input).unwrap();
    let (_, moves) = parse_moves(input).unwrap();

    let mut piles: Vec<Vec<char>> = vec![];
    crates.iter().rev().for_each(|row| {
        row.iter().enumerate().for_each(|(i, crate_value)| {
            if let Some(crate_value) = crate_value {
                if piles.len() < i + 1 {
                    piles.resize(i + 1, vec![]);
                }
                piles[i].push(*crate_value);
            };
        })
    });

    let mut new_piles = piles.clone();
    for move_instruction in &moves {
        for _ in 0..move_instruction.amount {
            let value = new_piles[(move_instruction.from - 1) as usize].pop();
            if let Some(value) = value {
                (new_piles[(move_instruction.to - 1) as usize]).push(value);
            }
        }
    }
    println!("{}", get_top_row(&new_piles));

    let mut new_piles = piles.clone();
    for m in &moves {
        let len = new_piles[(m.from - 1) as usize].len();
        let mut to_move = new_piles[(m.from - 1) as usize]
            .drain((len - m.amount as usize)..len)
            .collect::<Vec<char>>();
        new_piles[(m.to - 1) as usize].append(&mut to_move);
    }
    println!("{}", get_top_row(&new_piles));
}
