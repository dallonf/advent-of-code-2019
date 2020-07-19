// Day NN: Description

use crate::prelude::*;

fn flatten_results<'a, T, E>(results: impl Iterator<Item = Result<T, E>>) -> Result<Vec<T>, Vec<E>>
where
  E: std::fmt::Debug,
{
  results.fold(Ok(vec![]), |prev, next| match prev {
    Ok(mut results) => match next {
      Ok(next_result) => {
        results.push(next_result);
        Ok(results)
      }
      Err(next_err) => Err(vec![next_err]), // Start tracking errors instead
    },
    Err(mut errs) => match next {
      Ok(_next_result) => Err(errs), // ignore OK results
      Err(next_err) => {
        errs.push(next_err);
        Err(errs)
      }
    },
  })
}

#[derive(Debug, PartialEq)]
pub struct SpaceImage {
  pub layers: Vec<Layer>,
}

impl SpaceImage {
  pub fn parse(input: &str, width: usize, height: usize) -> Result<SpaceImage, String> {
    let layer_size = width * height;

    if input.len() % layer_size != 0 {
      return Err(
        format!(
          "Input must contain layers sized width:{} and height:{}",
          width, height
        )
        .into(),
      );
    }

    let layers = SpaceImageParseIterator {
      remaining_str: input,
      layer_size,
    };

    let layers = flatten_results(layers.map(|x| Layer::parse(x, width, height)));

    match layers {
      Ok(layers) => Ok(SpaceImage { layers }),
      Err(errs) => Err(errs.join("\n")),
    }
  }
}

struct SpaceImageParseIterator<'a> {
  remaining_str: &'a str,
  layer_size: usize,
}
impl<'a> Iterator for SpaceImageParseIterator<'a> {
  type Item = &'a str;
  fn next(&mut self) -> Option<Self::Item> {
    if self.remaining_str.len() >= self.layer_size {
      let (next_layer, remaining_str) = self.remaining_str.split_at(self.layer_size);
      self.remaining_str = remaining_str;
      Some(next_layer)
    } else {
      None
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Layer {
  pub rows: Vec<Vec<u8>>,
}

impl Layer {
  fn parse(input: &str, width: usize, height: usize) -> Result<Layer, String> {
    if input.len() != width * height {
      return Err(format!("Layer must be sized width:{} and height:{}", width, height).into());
    }

    let rows = (0..height).map(|y| {
      (0..width)
        .map(move |x| input.bytes().nth(y * width + x).unwrap())
        .map(move |b| {
          let c = char::from(b);
          u8::from_str_radix(&c.to_string(), 10).map_err(|err| err.to_string())
        })
        .collect()
    });

    let rows = flatten_results(rows);

    match rows {
      Ok(rows) => Ok(Layer { rows }),
      Err(errs) => Err(errs.join("\n")),
    }
  }
}

lazy_static! {
  static ref PUZZLE_INPUT: String = puzzle_input::string_for_day("08");
}

#[cfg(test)]
mod part_one {
  use super::*;

  #[test]
  fn flatten_result_success() {
    let input: Vec<Result<_, ()>> = vec![Ok("hello"), Ok("world")];
    let expected = Ok(vec!["hello", "world"]);
    assert_eq!(flatten_results(input.into_iter()), expected);
  }

  #[test]
  fn flatten_result_error() {
    let input: Vec<Result<(), _>> = vec![Err("error"), Err("world")];
    let expected = Err(vec!["error", "world"]);
    assert_eq!(flatten_results(input.into_iter()), expected);
  }

  #[test]
  fn flatten_result_mixed() {
    let input: Vec<Result<_, _>> = vec![Ok("hello"), Err("error"), Ok("universe"), Err("world")];
    let expected = Err(vec!["error", "world"]);
    assert_eq!(flatten_results(input.into_iter()), expected);
  }

  #[test]
  fn parse() {
    let input = "123456789012";
    let image = SpaceImage::parse(input, 3, 2).unwrap();
    assert_eq!(
      image,
      SpaceImage {
        layers: vec![
          Layer {
            rows: vec![vec![1, 2, 3], vec![4, 5, 6]]
          },
          Layer {
            rows: vec![vec![7, 8, 9], vec![0, 1, 2]]
          }
        ]
      }
    )
  }
  #[test]
  fn answer() {}
}

// #[cfg(test)]
// mod part_two {
//   use super::*;
//   #[test]
//   fn test_cases() {}
//   #[test]
//   fn answer() {}
// }
