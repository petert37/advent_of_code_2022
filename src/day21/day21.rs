use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, alpha1, line_ending},
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
enum Expression {
    Number(i64),
    Operation {
        lhs: String,
        rhs: String,
        operator: Operator,
    },
}

impl Expression {
    fn contains(&self, name: &String, monkeys: &HashMap<String, Monkey>) -> bool {
        match self {
            Expression::Number(_) => false,
            Expression::Operation {
                lhs,
                rhs,
                operator: _,
            } => {
                lhs == name
                    || rhs == name
                    || monkeys[lhs].expression.contains(name, monkeys)
                    || monkeys[rhs].expression.contains(name, monkeys)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    expression: Expression,
}

fn parse_monkeys(input: &str) -> IResult<&str, HashMap<String, Monkey>> {
    separated_list1(
        line_ending,
        tuple((
            alpha1,
            tag(": "),
            alt((
                complete::i64.map(|num| Expression::Number(num)),
                tuple((alpha1, tag(" "), take(1usize), tag(" "), alpha1)).map(
                    |(lhs, _, operator, _, rhs)| {
                        let operator = match operator {
                            "+" => Operator::Add,
                            "-" => Operator::Subtract,
                            "*" => Operator::Multiply,
                            "/" => Operator::Divide,
                            _ => panic!("Unknown operator: {}", operator),
                        };
                        Expression::Operation { lhs: lhs.to_string(), rhs: rhs.to_string(), operator }
                    },
                ),
            )),
        ))
        .map(|(name, _, expression)| Monkey { name: name.to_string(), expression }),
    )
    .map(|monkeys| {
        monkeys
            .into_iter()
            .map(|monkey| (monkey.name.clone(), monkey))
            .collect::<HashMap<String, Monkey>>()
    })
    .parse(input)
}

fn calculate(monkey: &Monkey, monkeys: &HashMap<String, Monkey>) -> i64 {
    match &monkey.expression {
        Expression::Number(value) => *value,
        Expression::Operation { lhs, rhs, operator } => {
            let lhs = calculate(&monkeys[lhs], monkeys);
            let rhs = calculate(&monkeys[rhs], monkeys);
            match operator {
                Operator::Add => lhs + rhs,
                Operator::Subtract => lhs - rhs,
                Operator::Multiply => lhs * rhs,
                Operator::Divide => lhs / rhs,
            }
        }
    }
}

fn solve(lhs: String, rhs: String, target: &String, monkeys: &HashMap<String, Monkey>) -> i64 {
    let mut lhs = lhs;
    let mut rhs = rhs;

    let mut monkeys = monkeys.clone();

    let mut i = 0;
    while &rhs != target {
        match &monkeys[&rhs].expression {
            Expression::Number(_) => panic!("Right hand side cannot be a number, must be an operation"),
            Expression::Operation {
                lhs: monkey_lhs,
                rhs: monkey_rhs,
                operator,
            } => {
                let new_monkey_name = format!("new_monkey_{}", i);
                let (new_expression, left) = if monkey_lhs == target || monkeys[monkey_lhs].expression.contains(target, &monkeys) {
                    let new_expression = match operator {
                        Operator::Add => Expression::Operation {
                            lhs: lhs,
                            rhs: monkey_rhs.clone(),
                            operator: Operator::Subtract,
                        },
                        Operator::Subtract => Expression::Operation {
                            lhs: lhs,
                            rhs: monkey_rhs.clone(),
                            operator: Operator::Add,
                        },
                        Operator::Multiply => Expression::Operation {
                            lhs: lhs,
                            rhs: monkey_rhs.clone(),
                            operator: Operator::Divide,
                        },
                        Operator::Divide => Expression::Operation {
                            lhs: lhs,
                            rhs: monkey_rhs.clone(),
                            operator: Operator::Multiply,
                        },
                    };
                    (new_expression, true)
                } else if monkey_rhs == target || monkeys[monkey_rhs].expression.contains(target, &monkeys) {
                    let new_expression = match operator {
                        Operator::Add => Expression::Operation {
                            lhs: lhs,
                            rhs: monkey_lhs.clone(),
                            operator: Operator::Subtract,
                        },
                        Operator::Subtract => Expression::Operation {
                            lhs: monkey_lhs.clone(),
                            rhs: lhs,
                            operator: Operator::Subtract,
                        },
                        Operator::Multiply => Expression::Operation {
                            lhs: lhs,
                            rhs: monkey_lhs.clone(),
                            operator: Operator::Divide,
                        },
                        Operator::Divide => Expression::Operation {
                            lhs: monkey_lhs.clone(),
                            rhs: lhs,
                            operator: Operator::Divide,
                        },
                    };
                    (new_expression, false)
                } else {
                    panic!("Target must be on one of the sides of the operation")
                };
                let new_monkey = Monkey {
                    name: new_monkey_name.clone(),
                    expression: new_expression
                };
                lhs = new_monkey_name.clone();
                rhs = if left {
                    monkey_lhs.clone()
                } else {
                    monkey_rhs.clone()
                };
                monkeys.insert(new_monkey_name, new_monkey);
            }
        }
        i += 1;
    }

    calculate(&monkeys[&lhs], &monkeys)
}

fn main() {
    let input = fs::read_to_string("src/day21/input.txt").unwrap();
    let (_, monkeys) = parse_monkeys(&input).unwrap();

    let root = &monkeys["root"];
    let result = calculate(root, &monkeys);
    println!("Part 1: {}", result);

    let mut monkeys = monkeys.clone();

    let zero_name = "zero".to_string();
    let zero = Monkey {
        name: zero_name.clone(),
        expression: Expression::Number(0),
    };

    let to_solve_name = "to_solve".to_string();
    let to_solve = Monkey {
        name: to_solve_name.clone(),
        expression: if let Expression::Operation {
            lhs,
            rhs,
            operator: _,
        } = &root.expression
        {
            Expression::Operation {
                lhs: lhs.clone(),
                rhs: rhs.clone(),
                operator: Operator::Subtract,
            }
        } else {
            panic!("Root cannot have a number expression, must be an operation")
        },
    };

    monkeys.insert(zero.name.clone(), zero);
    monkeys.insert(to_solve.name.clone(), to_solve);

    let result = solve(zero_name, to_solve_name, &"humn".to_string(), &monkeys);
    println!("Part 2: {}", result);
}
