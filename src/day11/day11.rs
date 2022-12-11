use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult, Parser,
};

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
enum Operand {
    Old,
    Const(u64),
}

#[derive(Debug, Clone)]
struct Operation {
    operand1: Operand,
    operand2: Operand,
    operator: Operator,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    true_target: u64,
    false_target: u64,
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((
        tag("old").map(|_| Operand::Old),
        complete::u64.map(|value| Operand::Const(value)),
    ))(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _monkey_index) = terminated(
        preceded(tag("Monkey "), complete::u64),
        tuple((tag(":"), line_ending)),
    )(input)?;
    let (input, items) = terminated(
        preceded(
            tag("  Starting items: "),
            separated_list1(tag(", "), complete::u64),
        ),
        line_ending,
    )(input)?;
    let (input, (operand1, operator, operand2)) = terminated(
        preceded(
            tag("  Operation: new = "),
            tuple((
                parse_operand,
                alt((
                    tag(" + ").map(|_| Operator::Add),
                    tag(" * ").map(|_| Operator::Multiply),
                )),
                parse_operand,
            )),
        ),
        line_ending,
    )(input)?;
    let (input, test) = terminated(
        preceded(tag("  Test: divisible by "), complete::u64),
        line_ending,
    )(input)?;
    let (input, true_target) = terminated(
        preceded(tag("    If true: throw to monkey "), complete::u64),
        line_ending,
    )(input)?;
    let (input, false_target) =
        preceded(tag("    If false: throw to monkey "), complete::u64)(input)?;
    Ok((
        input,
        Monkey {
            items,
            operation: Operation {
                operand1,
                operand2,
                operator,
            },
            test,
            true_target,
            false_target,
        },
    ))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monkeys) = separated_list1(many1(line_ending), parse_monkey)(input)?;
    Ok((input, monkeys))
}

fn get_monkey_business(monkeys: &Vec<Monkey>, rounds: u32, decrease_worry: bool) -> u64 {
    let mut monkeys = monkeys.clone();
    let monkey_count = monkeys.len();
    let mut inspection_count: Vec<u64> = vec![0; monkey_count];

    let modulo: u64 = monkeys.iter().map(|m| m.test).product();

    for _round in 0..rounds {
        for monkey_index in 0..monkey_count {
            for item_index in 0..monkeys[monkey_index].items.len() {
                inspection_count[monkey_index] += 1;
                let (target_monkey, target_item) = {
                    let monkey = &monkeys[monkey_index];
                    let item = &monkey.items[item_index];
                    let operation = &monkey.operation;

                    let op1 = match &operation.operand1 {
                        Operand::Old => item.clone(),
                        Operand::Const(value) => *value,
                    };
                    let op2 = match &operation.operand2 {
                        Operand::Old => item.clone(),
                        Operand::Const(value) => *value,
                    };
                    let operation_result = match &operation.operator {
                        Operator::Add => op1 + op2,
                        Operator::Multiply => op1 * op2,
                    };

                    let result = if decrease_worry {
                        operation_result / 3
                    } else {
                        operation_result % modulo
                    };

                    let test_result = if &result % &monkey.test == 0 {
                        monkey.true_target
                    } else {
                        monkey.false_target
                    };

                    (test_result, result)
                };
                monkeys[target_monkey as usize].items.push(target_item);
            }
            monkeys[monkey_index].items.clear();
        }
    }

    inspection_count.sort_by(|a, b| b.cmp(a));
    return inspection_count.iter().take(2).product();
}

fn main() {
    let input = fs::read_to_string("src/day11/input.txt").unwrap();
    let (_, monkeys) = parse_monkeys(&input).unwrap();

    let monkey_business = get_monkey_business(&monkeys, 20, true);
    println!("Monkey business: {monkey_business}");

    let monkey_business = get_monkey_business(&monkeys, 10_000, false);
    println!("Monkey business: {monkey_business}");
}
