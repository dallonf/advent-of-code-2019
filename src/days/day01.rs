// Day 1: The Tyranny of the Rocket Equation

#[cfg(test)]
mod test {
  lazy_static! {
    static ref PUZZLE_INPUT: Vec<String> = crate::util::puzzle_input::load_input_for_day("01");
  }

  pub fn fuel_amount(mass: i64) -> i64 {
    (mass / 3) - 2
  }

  pub fn recursive_fuel_amount(mass: i64) -> i64 {
    let required_fuel = fuel_amount(mass);
    if required_fuel > 0 {
      let additional_fuel = recursive_fuel_amount(required_fuel);
      return required_fuel + additional_fuel;
    } else {
      return 0;
    };
  }

  mod part_one {
    use super::*;

    #[test]
    fn mass_of_12() {
      assert_eq!(fuel_amount(12), 2);
    }
    #[test]
    fn mass_of_14() {
      assert_eq!(fuel_amount(14), 2);
    }
    #[test]
    fn mass_of_1969() {
      assert_eq!(fuel_amount(1969), 654);
    }
    #[test]
    fn mass_of_100_756() {
      assert_eq!(fuel_amount(100_756), 33_583);
    }
    #[test]
    fn part_one() {
      let result: i64 = PUZZLE_INPUT
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .map(|num| fuel_amount(num))
        .sum();
      assert_eq!(result, 3394106);
    }
  }

  mod part_two {
    use super::*;
    #[test]
    fn test_cases() {
      assert_eq!(recursive_fuel_amount(14), 2);
      assert_eq!(recursive_fuel_amount(1969), 966);
      assert_eq!(recursive_fuel_amount(100756), 50346);
    }
    #[test]
    fn part_two() {
      let result: i64 = PUZZLE_INPUT
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .map(|num| recursive_fuel_amount(num))
        .sum();
      assert_eq!(result, 5088280);
    }
  }
}
