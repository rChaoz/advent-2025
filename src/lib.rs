mod days;

use crate::Part::{Both, One};
use crate::RunType::{Examples, Full};
use days::DAYS;
use io_tee::TeeWriter;
use std::fmt::Display;
use std::fs::File;
use std::io::{Read, Write, stdout};
use std::path::Path;
use std::time::Instant;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RunType {
    Examples = 1,
    Full = 2,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Part {
    One = 1,
    Both = 2,
}

pub struct DayInfo {
    /// Name of the day, e.g. "Secret Entrance" for Day 1
    pub name: &'static str,
    /// Day solution, use `result()` to output the final result
    pub run: fn(input: &mut Context) -> (),
    /// Example input
    pub example: &'static str,
}

pub struct Context<'a> {
    /// Problem input
    pub input: &'a str,
    /// Whether running just part 1 or both parts
    pub part: Part,
    /// Whether running an example or the full input.
    /// Should generally not be used, unless the problem statement clearly differs between the two.
    pub run_type: RunType,
    day: &'a DayInfo,
    result_count: u8,
    output: &'a mut dyn Write,
}

impl<'a> Context<'a> {
    fn result<T: Display>(&mut self, result: T) {
        if self.part == One && self.result_count > 0 {
            // do nothing if trying to provide part 2 answer when running only part 1
            return;
        } else if self.result_count > 1 {
            panic!("Cannot call `context.result()` more than twice")
        }
        self.result_count += 1;
        writeln!(self.output, "Part {} result: {}", self.result_count, result).unwrap();
    }
}

/// Helper that resolves to `println!()`, disabled in release mode
#[macro_export]
macro_rules! debug {
    ($($e:expr),+) => {
        {
            #[cfg(debug_assertions)]
            {
                println!($($e),+)
            }
        }
    };
}

/// Helper that resolves to `println!()`, disabled in release mode and when running on full data
#[macro_export]
macro_rules! debug_example {
    ($context:expr, $($e:expr),+) => {
        {
            #[cfg(debug_assertions)]
            {
                if ($context.run_type == crate::RunType::Examples) {
                    println!($($e),+)
                }
            }
        }
    };
}

/// Runs the solution for the given 1-indexed day number (1-25).
/// * `day` - day number (1-25)
/// * `part` - which part to run (1, 2 or both - 3)
/// * `run_type` - whether to run on the example code or full input
pub fn run_single(day: u8, part: Part, run_type: RunType) {
    if day == 0 {
        panic!("day cannot be 0")
    }
    if let Some(day_info) = DAYS.get(day as usize - 1) {
        println!("\nRunning single day {}: {}\n\n", day, day_info.name);
        match run_type {
            Examples => run_examples(day_info, part),
            Full => run_full(day, day_info, part),
        }
    } else {
        panic!("day {} not found, max day is {}", day, DAYS.len())
    }
}

/// Runs the solution for all days.
/// * `run_type` - whether to run on the example code or full input
pub fn run_all(run_type: RunType) {
    println!("\nRunning all days\n\n");
    for (day, day_info) in DAYS.iter().enumerate() {
        let day = (day + 1) as u8;
        println!("# Day {}: {}", day, day_info.name);
        match run_type {
            Examples => run_examples(day_info, Both),
            Full => run_full(day, day_info, Both),
        }
    }
}

fn run_day(context: &mut Context) {
    match context.part {
        One => println!("### Running part 1"),
        Both => println!("### Running both parts"),
    }
    let time = Instant::now();
    (context.day.run)(context);
    println!("### done in {:?}\n", time.elapsed());
    match context.part {
        One | Both if context.result_count == 0 => {
            panic!("context.result() must be called to output your final result")
        }
        Both if context.result_count == 1 => panic!(
            "context.result() must be called twice then context.part == Both, once for each part's output"
        ),
        _ => {}
    }
}

fn run_examples(day: &DayInfo, part: Part) {
    let mut output = stdout();
    let mut context = Context {
        input: day.example,
        run_type: Examples,
        part,
        day,
        result_count: 0,
        output: &mut output,
    };
    run_day(&mut context);
}

fn run_full(day_num: u8, day: &DayInfo, part: Part) {
    // Files: data/{day}/{input,output}.txt
    let path = Path::new("data").join(day_num.to_string());
    // Read input file
    let mut input = String::new();
    File::open(path.join("input.txt"))
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    // Open output file
    let output_file = File::create(path.join("output.txt")).unwrap();
    let mut output = TeeWriter::new(output_file, stdout());
    // Create context
    let mut context = Context {
        input: &input,
        run_type: Full,
        part,
        day,
        result_count: 0,
        output: &mut output,
    };
    run_day(&mut context);
}
