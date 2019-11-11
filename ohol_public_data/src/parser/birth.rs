use super::common::*;
use crate::{Birth, PlayerId};
use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::char,
  combinator::{map, value},
  sequence::{pair, tuple},
  IResult,
};

fn some_parent(i: &str) -> IResult<&str, Option<PlayerId>> {
  let parser = pair(tag("parent="), player_id);
  map(parser, |(_, id)| Some(id))(i)
}

fn no_parent(i: &str) -> IResult<&str, Option<PlayerId>> {
  value(None, tag("noParent"))(i)
}

fn parent(i: &str) -> IResult<&str, Option<PlayerId>> {
  alt((some_parent, no_parent))(i)
}

fn chain(i: &str) -> IResult<&str, usize> {
  let parser = pair(tag("chain="), unsigned);
  map(parser, |(_, chain)| chain)(i)
}

pub fn birth(i: &str) -> IResult<&str, Birth> {
  let __ = space;
  let parser = tuple((
    char('B'),
    __,
    unix_time,
    __,
    player_id,
    __,
    email_hash,
    __,
    sex,
    __,
    coords,
    __,
    parent,
    __,
    pop,
    __,
    chain,
  ));
  map(parser, |outputs| {
    let (
      _b,
      _,
      unix_time,
      _,
      player_id,
      _,
      email_hash,
      _,
      sex,
      _,
      coords,
      _,
      parent,
      _,
      pop,
      _,
      chain,
    ) = outputs;
    Birth {
      unix_time,
      player_id,
      email_hash,
      sex,
      coords,
      parent,
      pop,
      chain,
    }
  })(i)
}

#[cfg(test)]
mod tests {
  use super::*;

  static BIRTH_1: &'static str = "B 1573344045 2256651 c5c94e5501424d0567c90730f5e2e6ad482a440f M (-2923,-1233) parent=2256572 pop=92 chain=16";

  #[test]
  fn parsing_birth_succeeds() {
    assert!(birth(BIRTH_1).is_ok());
  }
}
