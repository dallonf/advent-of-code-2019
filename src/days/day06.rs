// Day 6: Universal Orbit Map

use crate::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

type OrbitMapDefinition = HashMap<String, String>;

pub struct OrbitMap {
  all_nodes: HashMap<String, OrbitMapNodeData>,
  root_node: String,
}
struct OrbitMapNodeData {
  id: String,
  orbiting_id: Option<String>,
  satellite_ids: HashSet<String>,
}
#[derive(Copy, Clone)]
pub struct OrbitMapNode<'a> {
  map: &'a OrbitMap,
  data: &'a OrbitMapNodeData,
}

impl OrbitMap {
  pub fn new(input: &[String]) -> OrbitMap {
    let orbits = parse_orbits(input);
    let mut map = OrbitMap {
      all_nodes: HashMap::new(),
      root_node: "COM".into(),
    };
    // Create nodes
    map.all_nodes.insert(
      "COM".into(),
      OrbitMapNodeData {
        id: "COM".into(),
        orbiting_id: None,
        satellite_ids: HashSet::new(),
      },
    );
    for (satellite, star) in orbits.iter() {
      map.all_nodes.insert(
        satellite.clone(),
        OrbitMapNodeData {
          id: satellite.clone(),
          orbiting_id: Some(star.clone()),
          satellite_ids: HashSet::new(),
        },
      );
    }

    // Precompute satellites
    for (satellite, star) in orbits.iter() {
      let star_data = map.all_nodes.get_mut(star).unwrap();
      star_data.satellite_ids.insert(satellite.into());
    }

    map
  }

  pub fn get_node(&self, name: &str) -> Option<OrbitMapNode> {
    let data = self.all_nodes.get(name);
    if let Some(data) = data {
      Some(OrbitMapNode { map: &self, data })
    } else {
      None
    }
  }

  pub fn get_root_node(&self) -> OrbitMapNode {
    self.get_node(&self.root_node).unwrap()
  }

  pub fn get_all_nodes(&self) -> impl Iterator<Item = OrbitMapNode> {
    self
      .all_nodes
      .keys()
      .map(move |x| self.get_node(x).unwrap())
  }

  pub fn checksum(&self) -> usize {
    self
      .get_all_nodes()
      .map(|x| x.count_orbits_recursively())
      .sum()
  }

  pub fn minimum_distance(source: &OrbitMapNode, destination: &OrbitMapNode) -> usize {
    // Walk all the way up from the destination to the root node,
    // keeping track of how many transfers it takes to get to each one.
    let destination_parents = {
      let mut map: HashMap<String, usize> = HashMap::new();
      fn walk_destination_parents(
        map: &mut HashMap<String, usize>,
        node: &OrbitMapNode,
        transfers: usize,
      ) {
        map.insert(node.id().into(), transfers);
        if let Some(orbiting) = node.get_orbiting() {
          walk_destination_parents(map, &orbiting, transfers + 1);
        }
      };
      walk_destination_parents(&mut map, destination, 0);
      map
    };

    // Then walk up from the source until we find a common parent
    fn walk_source_parents(
      node: &OrbitMapNode,
      transfers: usize,
      destination_parents: &HashMap<String, usize>,
    ) -> usize {
      if let Some(common_parent_transfers) = destination_parents.get(node.id()) {
        transfers + common_parent_transfers
      } else {
        let new_node = node.get_orbiting().unwrap();
        walk_source_parents(&new_node, transfers + 1, destination_parents)
      }
    }

    walk_source_parents(source, 0, &destination_parents)
  }
}

impl OrbitMapNode<'_> {
  pub fn id(&self) -> &str {
    &self.data.id
  }

  pub fn get_orbiting(&self) -> Option<OrbitMapNode> {
    if let Some(id) = &self.data.orbiting_id {
      self.map.get_node(&id)
    } else {
      None
    }
  }
  pub fn get_satellites(&self) -> Vec<OrbitMapNode> {
    self
      .data
      .satellite_ids
      .iter()
      .filter_map(|id| self.map.get_node(&id))
      .collect()
  }
  pub fn count_orbits_recursively(&self) -> usize {
    self
      .get_orbiting()
      .map(|x| 1 + x.count_orbits_recursively())
      .unwrap_or(0)
  }
}

fn parse_orbits(input: &[String]) -> OrbitMapDefinition {
  let mut result: OrbitMapDefinition = HashMap::new();
  for line in input {
    let line: Vec<_> = line.split(")").collect();
    let center = line[0];
    let orbiter = line[1];
    result.insert(orbiter.into(), center.into());
  }
  result
}

lazy_static! {
  static ref PUZZLE_INPUT: Vec<String> = puzzle_input::lines_for_day("06");
}

#[cfg(test)]
mod part_one {
  use super::*;

  #[test]
  fn test_case() {
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
    let orbit_map = OrbitMap::new(&test_data);

    assert_eq!(orbit_map.checksum(), 42);
  }
  #[test]
  fn answer() {
    let map = OrbitMap::new(&PUZZLE_INPUT);
    assert_eq!(map.checksum(), 119831);
  }
}

#[cfg(test)]
mod part_two {
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
K)L
K)YOU
I)SAN"
      .lines()
      .map(|x| x.into())
      .collect();

    let map = OrbitMap::new(&test_data);
    assert_eq!(
      OrbitMap::minimum_distance(
        &map.get_node("YOU").unwrap().get_orbiting().unwrap(),
        &map.get_node("SAN").unwrap().get_orbiting().unwrap()
      ),
      4
    );
  }
  #[test]
  fn answer() {
    let map = OrbitMap::new(&PUZZLE_INPUT);
    assert_eq!(
      OrbitMap::minimum_distance(
        &map.get_node("YOU").unwrap().get_orbiting().unwrap(),
        &map.get_node("SAN").unwrap().get_orbiting().unwrap()
      ),
      322
    );
  }
}
