use crate::{debug_example, Context, DayInfo};
use std::cmp::{max, min};
use std::ops::RangeInclusive;

pub const INFO: DayInfo = DayInfo {
    name: "Cafeteria",
    run,
    example: "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32",
};

fn run(context: &mut Context) {
    let mut lines = context.input.lines();
    let ranges = {
        let mut ranges = Vec::new();
        loop {
            let line = lines.next().unwrap();
            if line.is_empty() {
                break;
            }
            let mut nums = line.split('-');
            let from: u64 = nums.next().unwrap().parse().unwrap();
            let to: u64 = nums.next().unwrap().parse().unwrap();
            assert!(nums.next().is_none());
            assert!(to >= from);
            ranges.push(from..=to);
        }
        ranges
    };
    let ids = lines.map(|line| line.parse::<u64>().unwrap());
    let fresh = ids
        .filter(|id| {
            if ranges.iter().any(|range| range.contains(id)) {
                debug_example!(context, "{id}");
                true
            } else {
                false
            }
        })
        .count();
    context.result(fresh);

    // Contains sorted ranges; can be sorted since they never overlap
    let mut merged_ranges = Vec::<RangeInclusive<u64>>::new();
    for range in ranges {
        // Find first range that may intersect with current range
        let from = merged_ranges.partition_point(|r| r.end() < range.start());
        // Find first range past our current range
        let to = merged_ranges.partition_point(|r| r.start() <= range.end());

        // New range doesn't intersect with anything, insert it
        if from == to {
            merged_ranges.insert(from, range);
        } else {
            // Merge all ranges into one & replace intersecting ranges
            let merged = min(*range.start(), *merged_ranges[from].start())
                ..=max(*range.end(), *merged_ranges[to - 1].end());
            merged_ranges.drain(from + 1..to);
            merged_ranges[from] = merged;
        }
    }

    // Count all elements from merged ranges
    debug_example!(context, "{merged_ranges:?}");
    context.result(
        merged_ranges
            .iter()
            .map(|r| r.end() - r.start() + 1)
            .sum::<u64>(),
    );
}
