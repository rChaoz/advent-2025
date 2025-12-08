use crate::Part::One;
use crate::RunType::Examples;
use crate::{Context, DayInfo, debug_example};
use PointParseError::*;
use kust::ScopeFunctions;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

pub const INFO: DayInfo = DayInfo {
    name: "Playground",
    run,
    example: "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl FromStr for Point {
    type Err = PointParseError;

    /// Parses a point from a string of shape "x,y,z", where x, y and z are integers (i32 bounds).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn next_coord<'a>(
            it: &mut impl Iterator<Item = &'a str>,
            index: usize,
        ) -> Result<i32, PointParseError> {
            it.next()
                .ok_or(InvalidCoordsCount(index))?
                .parse()
                .map_err(|int_err| PointParseIntError(index, int_err))
        }

        let mut it = s.split(',');
        let x = next_coord(&mut it, 0)?;
        let y = next_coord(&mut it, 1)?;
        let z = next_coord(&mut it, 2)?;

        match it.next() {
            None => Ok(Self { x, y, z }),
            Some(_) => Err(InvalidCoordsCount(4 + it.count())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
enum PointParseError {
    /// Failed to parse the string segments with the specified index (as split by ',') into an integer
    #[error("could not parse string segment with index {0}")]
    PointParseIntError(usize, #[source] ParseIntError),
    /// String has invalid amount of segments (less or more than 3)
    #[error("string has invalid amount of coords: {0}, should have 3")]
    InvalidCoordsCount(usize),
}

/// A segment is defined by two points; the order of the points does not matter
#[derive(Debug, Clone, Eq)]
struct Segment(Point, Point);

impl Segment {
    fn length_sq(&self) -> u64 {
        (self.0.x as i64).abs_diff(self.1.x as i64).pow(2)
            + (self.0.y as i64).abs_diff(self.1.y as i64).pow(2)
            + (self.0.z as i64).abs_diff(self.1.z as i64).pow(2)
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} --- {}]", self.0, self.1)
    }
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Hash for Segment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Combines 2 coordinate values in a commutative way
        fn combine(a: i32, b: i32) -> i64 {
            let a = a as i64;
            let b = b as i64;
            a.wrapping_mul(b).wrapping_add(a).wrapping_add(b)
        }

        state.write_i64(combine(self.0.x, self.1.x));
        state.write_i64(combine(self.0.y, self.1.y));
        state.write_i64(combine(self.0.z, self.1.z));
    }
}

fn run(context: &mut Context) {
    let points = context
        .input
        .lines()
        .map(Point::from_str)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())
        .unwrap();
    let initial_connections_count = if context.run_type == Examples {
        10
    } else {
        1000
    };

    // All possible segments, sorted by their length
    let segments = points
        .iter()
        .enumerate()
        .flat_map(|(index, &p1)| {
            points[index + 1..].iter().map(move |&p2| {
                let segment = Segment(p1, p2);
                let length = segment.length_sq();
                (segment, length)
            })
        })
        .collect::<Vec<_>>()
        .apply(|v| v.sort_by_key(|(_, length)| *length));

    // Connect segments, forming circuits
    // For each point, remember the circuit ID
    let mut next_circuit = 0u32;
    let mut circuit_count = points.len();
    let mut point_to_circuit: HashMap<Point, u32> = HashMap::new();

    fn process_segment(
        #[allow(unused_variables)] context: &Context,
        segment: &Segment,
        point_to_circuit: &mut HashMap<Point, u32>,
        next_circuit: &mut u32,
        circuit_count: &mut usize,
    ) {
        match (
            point_to_circuit.get(&segment.0),
            point_to_circuit.get(&segment.1),
        ) {
            (Some(&circuit), None) => {
                point_to_circuit.insert(segment.1, circuit);
                debug_example!(context, "connecting {segment}; left was circuit {circuit}");
                *circuit_count -= 1;
            }
            (None, Some(&circuit)) => {
                point_to_circuit.insert(segment.0, circuit);
                debug_example!(context, "connecting {segment}; right was circuit {circuit}");
                *circuit_count -= 1;
            }
            (None, None) => {
                point_to_circuit.insert(segment.0, *next_circuit);
                point_to_circuit.insert(segment.1, *next_circuit);
                debug_example!(
                    context,
                    "connecting {segment}; new circuit {}",
                    *next_circuit
                );
                *next_circuit += 1;
                *circuit_count -= 1;
            }
            (Some(&circuit1), Some(&circuit2)) => {
                if circuit1 != circuit2 {
                    debug_example!(
                        context,
                        "connecting {segment}; found circuits {circuit1} and {circuit2} (merging into {circuit1})"
                    );
                    for circuit in point_to_circuit.values_mut() {
                        if *circuit == circuit2 {
                            *circuit = circuit1;
                        }
                    }
                    *circuit_count -= 1;
                } else {
                    debug_example!(context, "already connected {segment} on circuit {circuit1}");
                }
            }
        };
    }

    for (segment, _) in &segments[..initial_connections_count] {
        process_segment(
            context,
            segment,
            &mut point_to_circuit,
            &mut next_circuit,
            &mut circuit_count,
        );
    }

    let circuit_sizes = {
        let mut sizes = vec![0u32; next_circuit as usize];
        for &circuit in point_to_circuit.values() {
            sizes[circuit as usize] += 1
        }
        sizes.sort();
        sizes.reverse();
        sizes
    };
    context.result(
        circuit_sizes
            .iter()
            .take(3)
            .map(|n| *n as u64)
            .product::<u64>(),
    );

    if context.part == One {
        return;
    }

    for (segment, _) in &segments[initial_connections_count..] {
        process_segment(
            context,
            segment,
            &mut point_to_circuit,
            &mut next_circuit,
            &mut circuit_count,
        );
        if circuit_count == 1 {
            debug_example!(context, "connected all circuits at {segment}");
            context.result(segment.0.x * segment.1.x);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod segment {
        use super::*;
        use std::hash::{BuildHasher, RandomState};

        const P1: Point = Point { x: 1, y: 2, z: 3 };
        const P2: Point = Point { x: 4, y: 5, z: 6 };

        #[test]
        fn eq_commutative() {
            assert_eq!(Segment(P1, P2), Segment(P2, P1));
        }

        #[test]
        fn hash_commutative() {
            let state = RandomState::new();
            let mut h1 = state.build_hasher();
            let mut h2 = state.build_hasher();
            Segment(P1, P2).hash(&mut h1);
            Segment(P2, P1).hash(&mut h2);
            assert_eq!(h1.finish(), h2.finish());
        }
    }
}
