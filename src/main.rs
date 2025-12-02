use aoc_2025_rust::RunType::{Examples, Full};
use aoc_2025_rust::*;
use std::io;
use std::io::BufRead;

fn main() {
    println!("Advent of Code 2023: Rust");
    println!("Enter which day/part you would like to run, and in which mode (full/examples).");
    println!("- full mode - run using input/output files");
    println!("- examples mode - run using example input, outputting to stdout");
    println!();
    println!("Format: [<day>] [<mode>]");
    println!("  day - day number (1-25), or \"all\" (default)");
    println!("  mode - one of: f, f1, e, e1 (defaults to f, only f/e allowed if day is \"all\"):");
    println!("    - f1/f - run part(s) 1/both in full mode");
    println!("    - e1/e - run part(s) 1/both in examples mode");

    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    if line.trim().is_empty() {
        run_all(Full);
        return;
    }

    let mut words = line.split_whitespace();

    let day = match words.next() {
        None | Some("all") => {
            run_all(match words.next().unwrap_or("f") {
                "f" => Full,
                "e" => Examples,
                _ => panic!("invalid mode"),
            });
            return;
        }
        Some(day) => day.parse::<u8>().unwrap(),
    };

    let (part, run_type) = match words.next().unwrap_or("f") {
        "f" => (Part::Both, Full),
        "f1" => (Part::One, Full),
        "e" => (Part::Both, Examples),
        "e1" => (Part::One, Examples),
        _ => panic!("invalid mode"),
    };

    run_single(day, part, run_type);
}
