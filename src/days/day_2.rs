use crate::RunType::Examples;
use crate::{debug_example, Context, DayInfo};
use std::ops::Add;

pub const INFO: DayInfo = DayInfo {
    name: "Gift Shop",
    run,
    example: "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124",
};

fn is_invalid_id_part1(id: u64) -> bool {
    let str = id.to_string();
    if str.len() % 2 == 1 {
        false
    } else {
        let l = str.len() / 2;
        str[..l] == str[l..]
    }
}

fn is_invalid_id_part2(id: u64) -> bool {
    let str = id.to_string();
    if str.len() == 1 {
        return false;
    }
    'outer: for len in 1..=str.len() / 2 {
        if str.len() % len != 0 {
            continue;
        }
        let (part, mut rest) = str.split_at(len);
        while rest.len() > 0 {
            if !rest.starts_with(part) {
                continue 'outer;
            }
            rest = &rest[len..];
        }
        return true;
    }
    false
}

fn calc_sum<'a, R, F>(context: &Context, ids: R, product_id_check: F) -> u64
where
    R: Iterator<Item = u64>,
    F: Fn(u64) -> bool,
{
    ids.filter_map(|product_id| {
        if product_id_check(product_id) {
            debug_example!(context, "{product_id}");
            Some(product_id)
        } else {
            None
        }
    })
    .reduce(u64::add)
    .unwrap_or(0)
}

fn run(context: &mut Context) {
    // Example splits the input across multiple lines, not present in full input
    let input = if context.run_type == Examples {
        &context
            .input
            .lines()
            .fold(String::new(), |acc, line| acc + line)
    } else {
        context.input
    };
    let ids = input.split(',').flat_map(|seq| {
        let mut it = seq.split('-');
        let from: u64 = it.next().unwrap().parse().unwrap();
        let to: u64 = it.next().unwrap().parse().unwrap();
        assert!(it.next().is_none());
        assert!(from < to);
        from..=to
    });
    context.result(calc_sum(context, ids.clone(), is_invalid_id_part1));
    context.result(calc_sum(context, ids, is_invalid_id_part2));
}
