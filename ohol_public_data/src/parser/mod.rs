mod birth;
mod common;
mod death;

use crate::{LifeLog, LifeLogEntry};
use birth::*;
use death::*;
use nom::{
  branch::alt,
  character::complete::{line_ending, not_line_ending},
  combinator::{all_consuming, map},
  multi::many0,
  sequence::pair,
  IResult,
};

fn birth_entry(i: &str) -> IResult<&str, LifeLogEntry> {
  map(birth, LifeLogEntry::Birth)(i)
}

fn death_entry(i: &str) -> IResult<&str, LifeLogEntry> {
  map(death, LifeLogEntry::Death)(i)
}

fn life_log_entry(i: &str) -> IResult<&str, LifeLogEntry> {
  let (rem, (line, _eol)) = pair(not_line_ending, line_ending)(i)?;
  let entry = match all_consuming(alt((birth_entry, death_entry)))(line) {
    Ok(("", entry)) => entry,
    Ok(_) => unreachable!(),
    Err(_) => LifeLogEntry::Error(line.to_string()),
  };
  Ok((rem, entry))
}

pub fn life_log(i: &str) -> IResult<&str, LifeLog> {
  many0(life_log_entry)(i)
}
