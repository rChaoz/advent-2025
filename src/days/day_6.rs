use crate::{debug_example, Context, DayInfo};

pub const INFO: DayInfo = DayInfo {
    name: "Trash Compactor",
    run,
    example: "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ",
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Addition,
    Multiplication,
}

impl Operator {
    fn run(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Addition => a + b,
            Operator::Multiplication => a * b,
        }
    }

    fn identity(&self) -> u64 {
        match self {
            Operator::Addition => 0,
            Operator::Multiplication => 1,
        }
    }
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '+' => Operator::Addition,
            '*' => Operator::Multiplication,
            _ => panic!("Unknown operator: {value}"),
        }
    }
}

fn run(context: &mut Context) {
    // Part 1
    let (operators, numbers) = {
        let mut numbers = context
            .input
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let operators = numbers.pop().unwrap();
        (
            operators
                .into_iter()
                .map(|op| op.chars().next().unwrap().into())
                .collect::<Vec<Operator>>(),
            numbers
                .into_iter()
                .map(|line| {
                    line.into_iter()
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<_>>(),
        )
    };
    context.result(
        operators
            .iter()
            .enumerate()
            .map(|(index, operator)| {
                let result = numbers.iter().fold(operator.identity(), |acc, line| {
                    operator.run(acc, line[index])
                });
                debug_example!(context, "col {index}: {result}");
                result
            })
            .sum::<u64>(),
    );

    // Part 2
    let (operators, number_lines) = {
        let mut lines = context
            .input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        (lines.pop().unwrap(), lines)
    };
    let max_cols = number_lines.iter().map(Vec::len).max().unwrap();
    let mut col = 0usize;
    let mut sum = 0u64;
    while col < max_cols {
        let operator: Operator = operators[col].into();
        let mut acc = operator.identity();
        while number_lines
            .iter()
            .any(|line| col < line.len() && !line[col].is_whitespace())
        {
            let mut num = 0u64;
            for line in number_lines.iter() {
                if col < line.len() && !line[col].is_whitespace() {
                    num = num * 10 + line[col].to_digit(10).unwrap() as u64;
                }
            }
            acc = operator.run(acc, num);
            col += 1;
        }
        debug_example!(context, "col {col}: {acc}");
        sum += acc;
        col += 1;
    }
    context.result(sum);
}
