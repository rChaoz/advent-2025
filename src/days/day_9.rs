use crate::Part::One;
use crate::{Context, DayInfo, debug_example};
use Turn::*;
use displaythis::Display;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::num::ParseIntError;
use std::ops::AddAssign;
use std::str::FromStr;
use thiserror::Error;

pub const INFO: DayInfo = DayInfo {
    name: "Movie Theater",
    run,
    example: "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
#[display("{x},{y}")]
struct Point {
    x: i32,
    y: i32,
}

impl AddAssign<Self> for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn get_coord<'a>(
            it: &mut impl Iterator<Item = &'a str>,
            index: usize,
        ) -> Result<i32, PointParseError> {
            it.next()
                .ok_or(PointParseError::CoordCount(index))?
                .trim()
                .parse::<i32>()
                .map_err(|e| PointParseError::ParseInt(index, e))
        }

        let mut it = s.split(',');
        let x = get_coord(&mut it, 0)?;
        let y = get_coord(&mut it, 1)?;
        if it.next().is_some() {
            Err(PointParseError::CoordCount(3 + it.count()))
        } else {
            Ok(Point { x, y })
        }
    }
}

#[derive(Debug, Error)]
enum PointParseError {
    #[error("could not parse coord at index {0}")]
    ParseInt(usize, #[source] ParseIntError),
    #[error("expected 2 coords, got {0}")]
    CoordCount(usize),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Display)]
enum Turn {
    #[display("Clockwise")]
    Clockwise = 1,
    #[display("CounterClockwise")]
    CounterClockwise = -1,
}

impl TryFrom<i32> for Turn {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Clockwise),
            -1 => Ok(CounterClockwise),
            _ => Err(value),
        }
    }
}

impl From<Turn> for i32 {
    fn from(value: Turn) -> Self {
        value as i32
    }
}

/// Calculate if, when turning from p1-p2 to p2-p3, the turn is clockwise or counter-clockwise
//noinspection DuplicatedCode
fn calc_turn(p1: Point, p2: Point, p3: Point) -> Turn {
    if p1.x == p2.x {
        // Walking vertically, the next point be horizontal to point 2
        assert_eq!(p2.y, p3.y, "turn is not 90 degrees");
        // Down then left, or up and right
        if (p2.y > p1.y && p3.x < p2.x) || (p2.y < p1.y && p3.x > p2.x) {
            Clockwise
        } else {
            CounterClockwise
        }
    } else if p1.y == p2.y {
        // Walking horizontally, the next point be vertical to point 2
        assert_eq!(p2.x, p3.x, "turn is not 90 degrees");
        // Right then down, or left and up
        if (p2.x > p1.x && p3.y > p2.y) || (p2.x < p1.x && p3.y < p2.y) {
            Clockwise
        } else {
            CounterClockwise
        }
    } else {
        panic!("points {p1} and {p2} are not adjacent")
    }
}

fn identify_largest_rect_area(
    points: &Vec<Point>,
    validate_rect: impl Fn(Point, Point) -> bool,
) -> u64 {
    points
        .iter()
        .enumerate()
        .filter_map(|(index, p1)| {
            points[index + 1..]
                .iter()
                .filter_map(|p2| {
                    if !validate_rect(
                        Point {
                            x: min(p1.x, p2.x),
                            y: min(p1.y, p2.y),
                        },
                        Point {
                            x: max(p1.x, p2.x),
                            y: max(p1.y, p2.y),
                        },
                    ) {
                        None
                    } else {
                        let width = p1.x.abs_diff(p2.x) + 1;
                        let height = p1.y.abs_diff(p2.y) + 1;
                        Some((width as u64) * (height as u64))
                    }
                })
                .max()
        })
        .max()
        .unwrap()
}

fn calc_outside_segment_diff(p1: Point, p2: Point, loop_direction: Turn) -> Point {
    if p1.x == p2.x {
        if (p2.y > p1.y && loop_direction == Clockwise)
            || (p2.y < p1.y && loop_direction == CounterClockwise)
        {
            Point { x: 1, y: 0 }
        } else if (p2.y < p1.y && loop_direction == Clockwise)
            || (p2.y > p1.y && loop_direction == CounterClockwise)
        {
            Point { x: -1, y: 0 }
        } else {
            panic!("unknown condition: {p1}, {p2}, turn = {loop_direction}");
        }
    } else if p1.y == p2.y {
        if (p2.x > p1.x && loop_direction == Clockwise)
            || (p2.x < p1.x && loop_direction == CounterClockwise)
        {
            Point { x: 0, y: -1 }
        } else if (p2.x < p1.x && loop_direction == Clockwise)
            || (p2.x > p1.x && loop_direction == CounterClockwise)
        {
            Point { x: 0, y: 1 }
        } else {
            panic!("unknown condition: {p1}, {p2}, turn = {loop_direction}");
        }
    } else {
        panic!("points {p1} and {p2} are not adjacent")
    }
}

fn run(context: &mut Context) {
    let points = context
        .input
        .lines()
        .map(Point::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    context.result(identify_largest_rect_area(&points, |_, _| true));
    let n = points.len();
    if context.part == One {
        return;
    }

    // Calculate total turn.
    let turn_count = (0..n)
        .map(|index| {
            i32::from(calc_turn(
                points[index % n],
                points[(index + 1) % n],
                points[(index + 2) % n],
            ))
        })
        .sum::<i32>();
    assert!(turn_count == 4 || turn_count == -4);

    // If the loop is clockwise, the count will be 4 more clockwise turns than counter-clockwise.
    // If the loop is counter-clockwise, the count will be 4 more counter-clockwise turns.
    let loop_direction: Turn = turn_count.signum().try_into().unwrap();

    // Calculate outside points
    let mut outside_points: HashSet<Point> = HashSet::new();
    for index in 0..n {
        // For each segment
        let mut p1 = points[index % n];
        let mut p2 = points[(index + 1) % n];
        let diff = calc_outside_segment_diff(p1, p2, loop_direction);
        p1 += diff;
        p2 += diff;
        let dir = Point {
            x: (p2.x - p1.x).signum(),
            y: (p2.y - p1.y).signum(),
        };
        loop {
            // Check if the point is on any edge
            let mut on_edge = false;
            for index_edge in 0..n {
                let e1 = points[index_edge % n];
                let e2 = points[(index_edge + 1) % n];
                if e1.x == e2.x
                    && e1.x == p1.x
                    && ((e1.y <= p1.y && p1.y <= e2.y) || e1.y >= p1.y && p1.y >= e2.y)
                {
                    on_edge = true;
                    break;
                } else if e1.y == e2.y
                    && e1.y == p1.y
                    && ((e1.x <= p1.x && p1.x <= e2.x) || (e1.x >= p1.x && p1.x >= e2.x))
                {
                    on_edge = true;
                    break;
                }
            }
            if !on_edge {
                outside_points.insert(p1);
            }
            if p1 == p2 {
                break;
            }
            p1 += dir;
        }
    }

    debug_example!(context, "{outside_points:#?}");

    context.result(identify_largest_rect_area(
        &points,
        |top_left, bottom_right| {
            !outside_points.iter().any(|p| {
                top_left.x <= p.x
                    && p.x <= bottom_right.x
                    && top_left.y <= p.y
                    && p.y <= bottom_right.y
            })
        },
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_points() -> Vec<Point> {
        INFO.example
            .lines()
            .map(Point::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    #[test]
    fn test_calc_turn() {
        let points = get_points();

        let turns = [
            Clockwise,
            Clockwise,
            Clockwise,
            CounterClockwise,
            Clockwise,
            Clockwise,
            CounterClockwise,
            Clockwise,
        ];

        assert_eq!(turns.len(), points.len());

        for (index, turn) in turns.iter().enumerate() {
            let p1 = points[(index) % points.len()];
            let p2 = points[(index + 1) % points.len()];
            let p3 = points[(index + 2) % points.len()];
            assert_eq!(
                calc_turn(p1, p2, p3),
                *turn,
                "turn at index {index} is incorrect: {p1} -> {p2} -> {p3} should be {turn}",
            )
        }
    }

    #[test]
    fn test_calc_outside_segment_diff() {
        let points = get_points();

        let diffs = [
            Point { x: 0, y: -1 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 0 },
            Point { x: 0, y: -1 },
            Point { x: -1, y: 0 },
        ];

        assert_eq!(diffs.len(), points.len());

        for (index, diff) in diffs.iter().enumerate() {
            let p1 = points[(index) % points.len()];
            let p2 = points[(index + 1) % points.len()];
            assert_eq!(
                calc_outside_segment_diff(p1, p2, Clockwise),
                *diff,
                "outside_diff at index {index} is incorrect: {p1} -> {p2} should be {diff}",
            )
        }
    }
}
