use crate::days::day_7::Tile::{Laser, Splitter, Start};
use crate::{Context, DayInfo};

pub const INFO: DayInfo = DayInfo {
    name: "Laboratories",
    run,
    example: "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    Empty,
    Splitter(Option<u64>),
    Laser,
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Start),
            '.' => Ok(Tile::Empty),
            '^' => Ok(Splitter(None)),
            _ => Err(value),
        }
    }
}

type Map = Vec<Vec<Tile>>;

fn run(context: &mut Context) {
    let map: Map = context
        .input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| Tile::try_from(char).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (start_x, start_y) = map
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            match line.iter().enumerate().find_map(|(x, &tile)| match tile {
                Start => Some(x),
                _ => None,
            }) {
                Some(x) => Some((x, y)),
                None => None,
            }
        })
        .unwrap();

    fn simulate_laser(map: &mut Map, start_x: usize, start_y: usize) -> u32 {
        let mut splits = 0u32;

        let x = start_x;
        for y in start_y..map.len() {
            match map[y][x] {
                Splitter(_) => {
                    splits += 1;
                    if x > 0 {
                        splits += simulate_laser(map, x - 1, y);
                    }
                    if x < map[0].len() - 1 {
                        splits += simulate_laser(map, x + 1, y);
                    }
                    break;
                }
                Laser => break,
                Tile::Empty => {
                    map[y][x] = Laser;
                }
                _ => {}
            }
        }
        splits
    }

    fn simulate_quantum_laser(map: &mut Map, start_x: usize, start_y: usize) -> u64 {
        let x = start_x;
        for y in start_y..map.len() {
            match map[y][x] {
                Splitter(Some(known_timelines)) => {
                    return known_timelines;
                }
                Splitter(None) => {
                    let mut timelines = 0u64;
                    if x > 0 {
                        timelines += simulate_quantum_laser(map, x - 1, y);
                    }
                    if x < map[0].len() - 1 {
                        timelines += simulate_quantum_laser(map, x + 1, y);
                    }
                    map[y][x] = Splitter(Some(timelines));
                    return timelines;
                }
                _ => {}
            }
        }
        1
    }

    context.result(simulate_laser(&mut map.clone(), start_x, start_y));
    context.result(simulate_quantum_laser(&mut map.clone(), start_x, start_y));
}
