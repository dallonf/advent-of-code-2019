// Day 6: Universal Orbit Map

use crate::prelude::*;
use std::collections::HashMap;

type OrbitMap = HashMap<String, String>;

pub fn parse_orbits(input: &[String]) -> OrbitMap {
  let mut result: OrbitMap = HashMap::new();
  for line in input {
    let line: Vec<_> = line.split(")").collect();
    let center = line[0];
    let orbiter = line[1];
    result.insert(orbiter.into(), center.into());
  }
  result
}

fn get_orbits(map: &OrbitMap, obj_id: &str) -> usize {
  match map.get(obj_id) {
    Some(orbiting_id) => 1 + get_orbits(map, orbiting_id),
    None => 0,
  }
}

pub fn checksum(map: &OrbitMap) -> usize {
  map.keys().map(|obj_id| get_orbits(map, obj_id)).sum()
}

lazy_static! {
  static ref PUZZLE_INPUT: Vec<String> = puzzle_input::lines_for_day("06");
}

#[cfg(test)]
mod part_one {
  use super::*;

  #[test]
  fn test_cases() {
    let test_data: Vec<_> = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"
      .lines()
      .map(|x| x.into())
      .collect();
    let orbit_map = parse_orbits(&test_data);

    assert_eq!(checksum(&orbit_map), 42);
  }
  #[test]
  fn answer() {
    let map = parse_orbits(&PUZZLE_INPUT);
    assert_eq!(checksum(&map), 119831);
  }
}

// #[cfg(test)]
// mod part_two {
//   use super::*;
//   #[test]
//   fn test_cases() {}
//   #[test]
//   fn answer() {}
// }
