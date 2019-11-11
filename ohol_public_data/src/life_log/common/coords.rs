use super::integer;
use nom::{
  character::complete::char, combinator::map, sequence::tuple, IResult,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Coords(isize, isize);

pub fn coords(i: &str) -> IResult<&str, Coords> {
  let parser = tuple((char('('), integer, char(','), integer, char(')')));
  map(parser, |(_, x, _, y, _)| Coords(x, y))(i)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parsing_coords_succeeds() {
    assert_eq!(coords("(-2923,-1233)"), Ok(("", Coords(-2923, -1233))));
  }
}
