use crate::{Coords, PlayerId, Sex};
use chrono::NaiveDateTime;
use nom::{
  branch::alt,
  bytes::complete::{tag, take, take_while_m_n},
  character::complete::{char, digit1},
  combinator::{map, map_opt, map_res, opt, value},
  sequence::{pair, tuple},
  IResult,
};

pub fn space(i: &str) -> IResult<&str, ()> {
  value((), char(' '))(i)
}

pub fn unsigned(i: &str) -> IResult<&str, usize> {
  map_res(digit1, |id: &str| id.parse::<usize>())(i)
}

pub fn integer(i: &str) -> IResult<&str, isize> {
  let parser = pair(opt(char('-')), digit1);
  map_res(parser, |(minus, num)| {
    let minus = minus.map_or("", |_| "-");
    let number = minus.to_string() + num;
    number.parse::<isize>()
  })(i)
}

pub fn unix_time(i: &str) -> IResult<&str, NaiveDateTime> {
  map_opt(take(10usize), |unix_time: &str| {
    let unix_time = unix_time.parse::<usize>().ok()?;
    NaiveDateTime::from_timestamp_opt(unix_time as i64, 0)
  })(i)
}

pub fn player_id(i: &str) -> IResult<&str, PlayerId> {
  map(unsigned, |id| PlayerId(id))(i)
}

pub fn email_hash(i: &str) -> IResult<&str, String> {
  let parser = take_while_m_n(40, 40, |chr: char| chr.is_digit(16));
  map(parser, &str::to_string)(i)
}

pub fn sex(i: &str) -> IResult<&str, Sex> {
  let parser = alt((char('F'), char('M')));
  map(parser, |chr| match chr {
    'F' => Sex::Female,
    'M' => Sex::Male,
    _ => unreachable!(),
  })(i)
}

pub fn coords(i: &str) -> IResult<&str, Coords> {
  let parser = tuple((char('('), integer, char(','), integer, char(')')));
  map(parser, |(_, x, _, y, _)| Coords(x, y))(i)
}

pub fn pop(i: &str) -> IResult<&str, usize> {
  let parser = pair(tag("pop="), unsigned);
  map(parser, |(_, pop)| pop)(i)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parsing_space_succeeds() {
    assert_eq!(space(" "), Ok(("", ())));
  }

  #[test]
  fn parsing_unsigned_succeeds() {
    assert_eq!(unsigned("123"), Ok(("", 123)));
  }

  #[test]
  fn parsing_integer_succeeds() {
    assert_eq!(integer("-123"), Ok(("", -123)));
  }

  #[test]
  fn parsing_unix_time_succeeds() {
    let expected_date =
      NaiveDateTime::parse_from_str("2019-11-10 00:00:45", "%Y-%m-%d %H:%M:%S")
        .unwrap();
    assert_eq!(unix_time("1573344045"), Ok(("", expected_date)));
  }

  #[test]
  fn parsing_player_id_succeeds() {
    assert_eq!(player_id("2256651"), Ok(("", PlayerId(2256651))));
  }

  #[test]
  fn parsing_email_hash_succeeds() {
    assert!(email_hash("c5c94e5501424d0567c90730f5e2e6ad482a440f").is_ok());
  }

  #[test]
  fn parsing_sex_succeeds() {
    assert_eq!(sex("F"), Ok(("", Sex::Female)));
    assert_eq!(sex("M"), Ok(("", Sex::Male)));
  }

  #[test]
  fn parsing_coords_succeeds() {
    assert_eq!(coords("(-2923,-1233)"), Ok(("", Coords(-2923, -1233))));
  }

  #[test]
  fn parsing_pop_succeeds() {
    assert_eq!(pop("pop=92"), Ok(("", 92)));
  }

  #[test]
  fn parsing_pop_fails() {
    assert!(pop("pop92").is_err());
  }
}
