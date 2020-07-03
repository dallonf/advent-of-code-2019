// Day 3: Crossed Wires

use crate::prelude::*;
use core::ops::{Add, Neg, Sub};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point(i32, i32);
pub struct WirePointData {
  steps_to_point: i32,
}
pub type Wire = HashMap<Point, WirePointData>;

impl Point {
  fn manhattan_distance_from_center(&self) -> i32 {
    self.0.abs() + self.1.abs()
  }
}

impl Add for Point {
  type Output = Point;
  fn add(self, rhs: Point) -> Self::Output {
    Point(self.0 + rhs.0, self.1 + rhs.1)
  }
}

impl Neg for Point {
  type Output = Point;
  fn neg(self) -> <Self as std::ops::Neg>::Output {
    Point(-self.0, -self.1)
  }
}

impl Sub for Point {
  type Output = Point;
  fn sub(self, rhs: Point) -> Self::Output {
    self + -rhs
  }
}

pub fn parse_wire(input: &str) -> Wire {
  let mut current_location = Point(0, 0);
  let mut result: Wire = HashMap::new();

  let turns = input.split(",");

  for turn in turns {
    let (direction, num) = turn.split_at(1);
    let num: i32 = num.parse().unwrap();
    let move_in_direction: Box<dyn Fn(Point) -> Point> = match direction {
      "U" => Box::new(|current_location| current_location - Point(0, 1)),
      "D" => Box::new(|current_location| current_location + Point(0, 1)),
      "L" => Box::new(|current_location| current_location - Point(1, 0)),
      "R" => Box::new(|current_location| current_location + Point(1, 0)),
      _ => panic!("Unexpected direction {}", direction),
    };

    for _ in 0..num {
      current_location = move_in_direction(current_location);
      result.insert(
        current_location,
        WirePointData {
          steps_to_point: result.len() as i32 + 1,
        },
      );
    }
  }

  result
}

pub fn get_closest_cross<'a>(wire1: &'a Wire, wire2: &'a Wire) -> Option<&'a Point> {
  let wire1_points: HashSet<_> = wire1.keys().collect();
  let wire2_points: HashSet<_> = wire2.keys().collect();
  let cross_points: Vec<&Point> = wire1_points.intersection(&wire2_points).cloned().collect();

  cross_points
    .iter()
    .min_by(|point1, point2| {
      point1
        .manhattan_distance_from_center()
        .cmp(&point2.manhattan_distance_from_center())
    })
    .cloned()
}

pub fn get_closest_cross_distance_from_wire_strings(wire1: &str, wire2: &str) -> Option<i32> {
  let wire1 = parse_wire(wire1);
  let wire2 = parse_wire(wire2);
  get_closest_cross(&wire1, &wire2).map(|x| x.manhattan_distance_from_center())
}

pub fn get_shortest_cross_time(wire1: &Wire, wire2: &Wire) -> Option<i32> {
  let wire1_points: HashSet<_> = wire1.keys().collect();
  let wire2_points: HashSet<_> = wire2.keys().collect();
  let cross_points: Vec<&Point> = wire1_points.intersection(&wire2_points).cloned().collect();

  cross_points
    .iter()
    .map(|point| {
      let total_steps = &wire1[point].steps_to_point + &wire2[point].steps_to_point;
      total_steps
    })
    .min()
}

pub fn get_shortest_cross_time_from_wire_strings(wire1: &str, wire2: &str) -> Option<i32> {
  let wire1 = parse_wire(wire1);
  let wire2 = parse_wire(wire2);
  get_shortest_cross_time(&wire1, &wire2)
}

lazy_static! {
  static ref PUZZLE_INPUT: Vec<String> = puzzle_input::lines_for_day("03");
}

#[cfg(test)]
mod part_one {
  use super::*;

  #[test]
  fn example() {
    let wire1 = parse_wire("R8,U5,L5,D3");
    let wire2 = parse_wire("U7,R6,D4,L4");
    let closest_cross = get_closest_cross(&wire1, &wire2);
    assert_eq!(
      closest_cross.map(|x| x.manhattan_distance_from_center()),
      Some(6)
    );
  }

  #[test]
  fn test_cases() {
    assert_eq!(
      get_closest_cross_distance_from_wire_strings(
        "R75,D30,R83,U83,L12,D49,R71,U7,L72",
        "U62,R66,U55,R34,D71,R55,D58,R83"
      ),
      Some(159)
    );
    assert_eq!(
      get_closest_cross_distance_from_wire_strings(
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
        "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
      ),
      Some(135)
    );
  }

  #[test]
  fn answer() {
    assert_eq!(
      get_closest_cross_distance_from_wire_strings(&PUZZLE_INPUT[0], &PUZZLE_INPUT[1]),
      Some(308)
    )
  }
}

#[cfg(test)]
mod part_two {
  use super::*;

  #[test]
  fn example() {
    let wire1 = parse_wire("R8,U5,L5,D3");
    let wire2 = parse_wire("U7,R6,D4,L4");
    let closest_cross = get_shortest_cross_time(&wire1, &wire2);
    assert_eq!(closest_cross, Some(30));
  }

  #[test]
  fn test_cases() {
    assert_eq!(
      get_shortest_cross_time_from_wire_strings(
        "R75,D30,R83,U83,L12,D49,R71,U7,L72",
        "U62,R66,U55,R34,D71,R55,D58,R83"
      ),
      Some(610)
    );
    assert_eq!(
      get_shortest_cross_time_from_wire_strings(
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
        "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
      ),
      Some(410)
    );
  }

  #[test]
  fn part_two() {
    assert_eq!(
      get_shortest_cross_time_from_wire_strings(&PUZZLE_INPUT[0], &PUZZLE_INPUT[1]),
      Some(12934)
    )
  }
}
