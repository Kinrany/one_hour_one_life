use super::super::common::{coords::*, player_id::*, sex::*, *};
use super::{Cause, Death};
use nom::{
  branch::alt,
  bytes::complete::{tag, take_while1},
  character::complete::char,
  combinator::{map, value},
  number::complete::float,
  sequence::{pair, tuple},
  IResult,
};

fn age(i: &str) -> IResult<&str, f32> {
  let parser = pair(tag("age="), float);
  map(parser, |(_, age)| age)(i)
}

fn cause_killer(i: &str) -> IResult<&str, Cause> {
  let parser = pair(tag("killer_"), player_id);
  map(parser, |(_, killer)| Cause::Killer(killer))(i)
}

fn cause_hunger(i: &str) -> IResult<&str, Cause> {
  value(Cause::Hunger, tag("hunger"))(i)
}

fn cause_old_age(i: &str) -> IResult<&str, Cause> {
  value(Cause::OldAge, tag("oldAge"))(i)
}

fn cause_disconnect(i: &str) -> IResult<&str, Cause> {
  value(Cause::Disconnect, tag("disconnect"))(i)
}

fn cause_other(i: &str) -> IResult<&str, Cause> {
  let parser = take_while1(|chr: char| !chr.is_whitespace());
  map(parser, |s: &str| Cause::Other(s.to_string()))(i)
}

fn cause(i: &str) -> IResult<&str, Cause> {
  alt((
    cause_killer,
    cause_hunger,
    cause_old_age,
    cause_disconnect,
    cause_other,
  ))(i)
}

pub fn death(i: &str) -> IResult<&str, Death> {
  let __ = space;
  let parser = tuple((
    char('D'),
    __,
    unix_time,
    __,
    player_id,
    __,
    email_hash,
    __,
    age,
    __,
    sex,
    __,
    coords,
    __,
    cause,
    __,
    pop,
  ));
  map(parser, |outputs| {
    let (
      _d,
      _,
      unix_time,
      _,
      player_id,
      _,
      email_hash,
      _,
      age,
      _,
      sex,
      _,
      coords,
      _,
      cause,
      _,
      pop,
    ) = outputs;
    Death {
      unix_time,
      player_id,
      email_hash,
      age,
      sex,
      coords,
      cause,
      pop,
    }
  })(i)
}

#[cfg(test)]
mod tests {
  use super::*;

  static DEATH_1: &'static str = "D 1573344001 2256479 4c5e506d41e214be98b621ae62dc93b977ca97e9 age=60.00 F (-1183,231) oldAge pop=91";

  #[test]
  fn parsing_death_succeeds() {
    assert!(death(DEATH_1).is_ok());
  }
}
