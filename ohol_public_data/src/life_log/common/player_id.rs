use super::unsigned;
use nom::{combinator::map, IResult};

#[derive(Clone, Debug, PartialEq)]
pub struct PlayerId(usize);

pub fn player_id(i: &str) -> IResult<&str, PlayerId> {
  map(unsigned, |id| PlayerId(id))(i)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parsing_player_id_succeeds() {
    assert_eq!(player_id("2256651"), Ok(("", PlayerId(2256651))));
  }
}
