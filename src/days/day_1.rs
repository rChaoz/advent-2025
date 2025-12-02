use crate::{debug_example, Context, DayInfo};

pub const INFO: DayInfo = DayInfo {
    name: "Secret Entrance",
    run,
    example: "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
};

fn run(context: &mut Context) {
    let rotations = context.input.lines().map(|line| {
        let multiplier = if line.chars().next().unwrap() == 'L' {
            -1
        } else {
            1
        };
        let amount = line[1..].parse::<i32>().unwrap();
        amount * multiplier
    });
    let mut dial = 50;
    let mut counter1 = 0;
    let mut counter2 = 0;
    for rotation in rotations {
        let new_dial = dial + rotation;
        let final_dial = new_dial.rem_euclid(100);
        if final_dial == 0 {
            debug_example!(context, "(1)  {rotation}");
            counter1 += 1;
        }
        let zero_counts = if new_dial > 0 {
            new_dial / 100
        } else if dial == 0 {
            new_dial / -100
        } else {
            1 + new_dial / -100
        };
        if zero_counts != 0 {
            debug_example!(context, "(2)  {rotation} - {zero_counts}");
        }
        counter2 += zero_counts;
        dial = final_dial;
    }
    context.result(counter1);
    context.result(counter2);
}
