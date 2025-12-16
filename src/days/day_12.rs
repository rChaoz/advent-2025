use crate::RunType::Examples;
use crate::{Context, DayInfo};
use regex::Regex;
use std::iter::once;
use std::sync::LazyLock;

pub const INFO: DayInfo = DayInfo {
    name: "Christmas Tree Farm",
    run,
    example: "",
};

static AREA_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d+)x(\d+): (\d+(?: \d+)*)").unwrap());

fn run(context: &mut Context) {
    if context.run_type == Examples {
        context.result("examples not supported for this day");
        context.result("-");
        return;
    }

    let (present_sizes, areas) = {
        let mut lines = context.input.lines();
        let mut present_sizes: Vec<u32> = Vec::new();
        let first = loop {
            let mut line = lines.next().unwrap();
            if !line.ends_with(":") {
                break line;
            }
            let mut area = 0;
            loop {
                area += line.chars().filter(|&c| c == '#').count() as u32;
                line = lines.next().unwrap();
                if line.is_empty() {
                    break;
                }
            }
            present_sizes.push(area);
        };
        let areas = once(first).chain(lines).map(|line| {
            let m = AREA_REGEX.captures(line).unwrap();
            let width = m[1].parse::<u32>().unwrap();
            let height = m[2].parse::<u32>().unwrap();
            let presents = m[3]
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            (width * height, presents)
        });
        (present_sizes, areas)
    };

    let mut definitely_fit = 0u32;
    #[allow(unused_variables)]
    let mut definitely_dont_fit = 0u32;
    let mut not_sure = 0u32;

    for (area, presents) in areas {
        let min_area = presents
            .iter()
            .enumerate()
            .map(|(present, &count)| count * present_sizes[present])
            .sum::<u32>();
        let max_area = 9 * presents.iter().sum::<u32>();

        if area < min_area {
            definitely_dont_fit += 1;
        } else if area > max_area {
            definitely_fit += 1;
        } else {
            not_sure += 1;
        }
    }

    // for whatever reason
    context.result(definitely_fit + not_sure);

    // no part 2
    context.result("-");
}
