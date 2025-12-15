use crate::Part::One;
use crate::{Context, DayInfo, debug_example};
use regex::Regex;
use std::cmp::min;
use std::fmt::Display;
use std::mem::swap;
use std::sync::LazyLock;

pub const INFO: DayInfo = DayInfo {
    name: "Factory",
    run,
    example: "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
};

static LINE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\[([.#]+?)]((?: \(\d+(?:,\d+)*\))+) \{(\d+(?:,\d+)*)}$").unwrap()
});

#[derive(Debug, Clone)]
struct Machine {
    indicators: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

#[allow(dead_code)]
fn print_matrix(matrix: &Vec<Vec<impl Display>>) {
    for row in matrix.iter() {
        for num in row {
            print!("{num}\t");
        }
        println!();
    }
}

/// Calculate the greatest common divisor
fn gcd(mut a: i32, mut b: i32) -> i32 {
    if b > a {
        swap(&mut a, &mut b);
    }
    loop {
        let r = a % b;
        if r == 0 {
            return b;
        }
        a = b;
        b = r;
    }
}

/// Calculate the least common multiple
fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

fn run(context: &mut Context) {
    let machines = context
        .input
        .lines()
        .map(|line| {
            let r = LINE_PATTERN.captures(line).unwrap();
            Machine {
                indicators: r[1].chars().map(|c| c == '#').collect::<Vec<_>>(),
                buttons: r[2]
                    .trim_start()
                    .split(' ')
                    .map(|s| {
                        assert!(s.starts_with('(') && s.ends_with(')'));
                        s[1..s.len() - 1]
                            .split(',')
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
                joltages: r[3]
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            }
        })
        .collect::<Vec<_>>();

    fn try_indicator_buttons(
        indicators: &Vec<bool>,
        expected_indicators: &Vec<bool>,
        buttons: &Vec<Vec<usize>>,
        button_index: usize,
        button_presses: u32,
    ) -> Option<u32> {
        debug_assert!(indicators.len() == expected_indicators.len());
        if indicators == expected_indicators {
            Some(button_presses)
        } else if button_index == buttons.len() {
            None
        } else {
            // Try without pressing the button
            let result1 = try_indicator_buttons(
                indicators,
                expected_indicators,
                buttons,
                button_index + 1,
                button_presses,
            );
            // Try with pressing the button
            let button = &buttons[button_index];
            let mut indicators = indicators.clone();
            for &index in button {
                indicators[index] = !indicators[index];
            }
            let result2 = try_indicator_buttons(
                &indicators,
                expected_indicators,
                buttons,
                button_index + 1,
                button_presses + 1,
            );
            // Prefer the result with the fewest button presses
            if let Some(r1) = result1
                && let Some(r2) = result2
            {
                Some(min(r1, r2))
            } else {
                result1.or(result2)
            }
        }
    }

    context.result(
        machines
            .iter()
            .map(|machine| {
                try_indicator_buttons(
                    &vec![false; machine.indicators.len()],
                    &machine.indicators,
                    &machine.buttons,
                    0,
                    0,
                )
                .unwrap()
            })
            .sum::<u32>(),
    );
    if context.part == One {
        return;
    }

    fn solve_machine(
        #[allow(unused_variables)] context: &Context,
        #[allow(unused_variables)] index: usize,
        machine: &Machine,
    ) -> u32 {
        // Convert buttons into system of equations
        let mut matrix = machine
            .joltages
            .iter()
            .enumerate()
            .map(|(index, &joltage)| {
                let mut row = machine
                    .buttons
                    .iter()
                    .map(|button| if button.contains(&index) { 1 } else { 0 })
                    .collect::<Vec<_>>();
                row.push(joltage as i32);
                row
            })
            .collect::<Vec<_>>();

        #[cfg(debug_assertions)]
        if context.run_type == crate::RunType::Examples {
            println!("\nInitial matrix [{}]:", index + 1);
            print_matrix(&matrix);
        }

        // Solve system using Gaussian elimination
        let mut i = 0;
        let mut j = 0;
        loop {
            if i >= matrix.len() || j >= matrix[0].len() {
                break;
            }
            // Skip if the column is all 0s, or a single non-0
            let mut non_zero_rows = matrix[i..].iter().enumerate().filter_map(|(index, row)| {
                if row[j] != 0 { Some(i + index) } else { None }
            });
            match non_zero_rows.next() {
                None => {
                    // Skip column
                    j += 1;
                    continue;
                }
                Some(swap_index) => {
                    // Swap with only non-0 row, if single
                    if non_zero_rows.next().is_none() {
                        matrix.swap(i, swap_index);
                        i += 1;
                        j += 1;
                        continue;
                    }
                }
            }
            // Need a 1 (or -1) at i,j
            if matrix[i][j].abs() != 1 {
                // Try to swap line with any other that has a 1/-1
                if let Some(swap_index) = matrix[i + 1..]
                    .iter()
                    .enumerate()
                    .find_map(|(index, row)| {
                        if row[j].abs() == 1 {
                            Some(i + 1 + index)
                        } else {
                            None
                        }
                    })
                    .or_else(|| {
                        // Can't find a 1, instead make sure we have a non-0
                        if matrix[i][j] == 0 {
                            matrix[i + 1..].iter().enumerate().find_map(|(index, row)| {
                                if row[j] != 0 {
                                    Some(i + 1 + index)
                                } else {
                                    None
                                }
                            })
                        } else {
                            None
                        }
                    })
                {
                    matrix.swap(i, swap_index);
                }
                if matrix[i][j].abs() != 1 {
                    // Couldn't find a 1. This is a problem because we want to work with integers. Why?
                    // For example, if current row has a 3, and another has 2, we will have to multiply current row with -2/3 to get the 0 we need,
                    // which will lead to fractions on other columns. Buttons have to be pressed an integer number of times, so we don't want fractions
                    // Also, floats are not good for fractions, so I'd have to bring a fraction library & deal with all that...
                    // Instead, multiply all other rows to reach a least common multiple
                    // In the example above, we'd multiply the "other" row by 3, leading to a 6, so we can multiply current row with -2 instead
                    let num = matrix[i][j];
                    for i2 in i + 1..matrix.len() {
                        if matrix[i2][j] == 0 {
                            continue;
                        }
                        let mul = lcm(num, matrix[i2][j]) / matrix[i2][j];
                        if mul != 1 {
                            for k in 0..matrix[0].len() {
                                matrix[i2][k] *= mul;
                            }
                        }
                    }
                }
            }
            // Make the rest of the column below 0s by adding the current row (times a multiplier) to it
            for i2 in (i + 1)..matrix.len() {
                if matrix[i2][j] != 0 {
                    // Lhs is always divisible by rhs because rhs is either 1/-1, or we dealt with it already (see the many comments few lines above)
                    let mul = -matrix[i2][j] / matrix[i][j];
                    for k in 0..matrix[0].len() {
                        matrix[i2][k] += mul * matrix[i][k];
                    }
                }
            }
            i += 1;
            j += 1;
        }

        #[cfg(debug_assertions)]
        if context.run_type == crate::RunType::Examples {
            println!("Row echelon form [{}]:", index + 1);
            print_matrix(&matrix);
        }

        // Finally, back-track through all solutions. This could definitely be optimized,
        // but it runs in ~0.01s (~1.5s for full input), so it's fine. Too much work already
        fn backtrack(
            // Constants
            context: &Context,
            machine: &Machine,
            matrix: &Vec<Vec<i32>>,
            // Current state
            row: isize,
            column: usize,
            remaining_sum: Option<i32>,
            button_presses: Vec<Option<u32>>,
        ) -> Option<u32> {
            if row < 0 {
                let sum = button_presses
                    .iter()
                    .filter_map(|&button| button)
                    .sum::<u32>();
                debug_example!(
                    context,
                    "Found {sum}: {:?}",
                    button_presses
                        .into_iter()
                        .map(Option::unwrap)
                        .collect::<Vec<_>>()
                );
                return Some(sum);
            }
            let remaining_sum = match remaining_sum {
                Some(sum) => sum,
                None => matrix[row as usize][matrix[0].len() - 1],
            };
            let buttons = &matrix[row as usize][0..matrix[0].len() - 1];
            let mut buttons_it = buttons[column..]
                .iter()
                .enumerate()
                .filter_map(|(index, &num)| if num != 0 { Some(column + index) } else { None });
            match buttons_it.next() {
                None => backtrack(context, machine, matrix, row - 1, 0, None, button_presses),
                Some(button) => {
                    if buttons_it.next().is_none() {
                        // No other buttons left, this button needs to finish the row's equation
                        if remaining_sum % buttons[button] == 0 {
                            let presses = remaining_sum / buttons[button];
                            if presses < 0 {
                                None
                            } else if let Some(known_presses) = button_presses[button]
                                && known_presses != presses as u32
                            {
                                None
                            } else {
                                let mut button_presses = button_presses.clone();
                                button_presses[button] = Some(presses as u32);
                                backtrack(
                                    context,
                                    machine,
                                    matrix,
                                    row - 1,
                                    0,
                                    None,
                                    button_presses,
                                )
                            }
                        } else {
                            None
                        }
                    } else {
                        // Other buttons left
                        if let Some(known_presses) = button_presses[button] {
                            // Number of presses is already known
                            backtrack(
                                context,
                                machine,
                                matrix,
                                row,
                                button + 1,
                                Some(remaining_sum - known_presses as i32 * buttons[button]),
                                button_presses,
                            )
                        } else {
                            // Try every possible number of presses
                            let mut best_result = u32::MAX;
                            // This could be heavily optimized. Right now it tries to push buttons like 0-50 times,
                            // although they have to be pressed 2-5 times, but I can't be bothered.
                            // Also, I tried optimizing it a bit, but it became slower so oh well
                            let max_presses = machine.buttons[button]
                                .iter()
                                .map(|&joltage_index| machine.joltages[joltage_index])
                                .min()
                                .unwrap() as i32;
                            for presses in 0..=max_presses {
                                let mut button_presses = button_presses.clone();
                                button_presses[button] = Some(presses as u32);
                                if let Some(result) = backtrack(
                                    context,
                                    machine,
                                    matrix,
                                    row,
                                    button + 1,
                                    Some(remaining_sum - presses * buttons[button]),
                                    button_presses,
                                ) && result < best_result
                                {
                                    best_result = result;
                                }
                            }
                            if best_result == u32::MAX {
                                None
                            } else {
                                Some(best_result)
                            }
                        }
                    }
                }
            }
        }

        backtrack(
            context,
            machine,
            &matrix,
            matrix.len() as isize - 1,
            0,
            None,
            vec![None; machine.buttons.len()],
        )
        .unwrap()
    }

    context.result(
        machines
            .iter()
            .enumerate()
            .map(|(index, machine)| solve_machine(context, index, machine))
            .sum::<u32>(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(12, 15), 3);
        assert_eq!(gcd(4, 34), 2);
        assert_eq!(gcd(12345, 1), 1);
    }
}
