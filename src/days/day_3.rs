use crate::{Context, DayInfo, debug_example};

pub const INFO: DayInfo = DayInfo {
    name: "Lobby",
    run,
    example: "\
987654321111111
811111111111119
234234234234278
818181911112111",
};

type Bank = Vec<u8>;

fn max_joltage1(bank: Bank) -> u64 {
    let first = *bank[..bank.len() - 1].iter().max().unwrap();
    let second = *bank[bank
        .iter()
        .enumerate()
        .find(|&(_, &v)| v == first)
        .unwrap()
        .0
        + 1..]
        .iter()
        .max()
        .unwrap();
    (first as u64) * 10 + (second as u64)
}

fn max_joltage2(bank: Bank) -> u64 {
    let mut joltage = 0;
    let mut first = 0usize;
    for digit_num in 1..=12usize {
        let (index, &digit) = bank[first..bank.len() - 12 + digit_num]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_, &v)| v)
            .unwrap();
        joltage = joltage * 10 + digit as u64;
        first += index + 1;
    }
    joltage
}

fn calc_total_joltage<T, F>(
    #[allow(unused_variables)] context: &Context,
    banks: T,
    get_max_joltage: F,
) -> u64
where
    T: Iterator<Item = Bank>,
    F: Fn(Bank) -> u64,
{
    banks
        .map(|bank| {
            let joltage = get_max_joltage(bank);
            debug_example!(context, "{joltage}");
            joltage
        })
        .sum()
}

fn run(context: &mut Context) {
    let banks = context.input.lines().map(|line| {
        line.chars()
            .map(|char| char.to_digit(10).unwrap() as u8)
            .collect::<Bank>()
    });
    context.result(calc_total_joltage(context, banks.clone(), max_joltage1));
    context.result(calc_total_joltage(context, banks, max_joltage2));
}
