use crate::days::day_4::Tile::{Empty, PaperRoll};
use crate::{debug_example, Context, DayInfo};

pub const INFO: DayInfo = DayInfo {
    name: "Printing Department",
    run,
    example: "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    PaperRoll,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Empty,
            '@' => PaperRoll,
            _ => unreachable!(),
        }
    }
}

fn is_accessible_roll(map: &Vec<Vec<Tile>>, roll_x: usize, roll_y: usize) -> bool {
    if map[roll_y][roll_x] != PaperRoll {
        return false;
    }

    let mut nearby_rolls: u32 = 0;
    // Check all nearby tiles
    for i in -1..=1 {
        for j in -1..=1 {
            let x = roll_x as i32 + i;
            let y = roll_y as i32 + j;
            if (i == 0 && j == 0)
                || x < 0
                || y < 0
                || x >= map[0].len() as i32
                || y >= map.len() as i32
            {
                continue;
            }
            if map[y as usize][x as usize] == PaperRoll {
                nearby_rolls += 1;
            }
        }
    }
    nearby_rolls < 4
}

fn run(context: &mut Context) {
    let mut map = context
        .input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Count accessible tiles (part 1)
    let mut accessible_count: u32 = 0;
    for y in 0..map[0].len() {
        for x in 0..map.len() {
            if is_accessible_roll(&map, x, y) {
                debug_example!(context, "accessible roll at x={x}, y={y}");
                accessible_count += 1;
            }
        }
    }

    context.result(accessible_count);

    // Remove rolls (part 2)
    let mut total_removed_rolls: u32 = 0;
    loop {
        let mut removed_rolls: u32 = 0;

        for y in 0..map[0].len() {
            for x in 0..map.len() {
                if is_accessible_roll(&map, x, y) {
                    map[y][x] = Empty;
                    removed_rolls += 1;
                    total_removed_rolls += 1;
                }
            }
        }

        if removed_rolls == 0 {
            break;
        }
    }
    context.result(total_removed_rolls);
}
