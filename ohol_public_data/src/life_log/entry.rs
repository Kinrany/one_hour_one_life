use super::{
  birth::{parser::birth, Birth},
  death::{parser::death, Death},
};
use nom::{
  branch::alt,
  character::complete::{line_ending, not_line_ending},
  combinator::{all_consuming, map},
  sequence::pair,
  IResult,
};

#[derive(Clone, Debug)]
pub enum LifeLogEntry {
  Birth(Birth),
  Death(Death),
  Error(String),
}

fn birth_entry(i: &str) -> IResult<&str, LifeLogEntry> {
  map(birth, LifeLogEntry::Birth)(i)
}

fn death_entry(i: &str) -> IResult<&str, LifeLogEntry> {
  map(death, LifeLogEntry::Death)(i)
}

pub fn life_log_entry(i: &str) -> IResult<&str, LifeLogEntry> {
  let (rem, (line, _eol)) = pair(not_line_ending, line_ending)(i)?;
  let entry = match all_consuming(alt((birth_entry, death_entry)))(line) {
    Ok((_, entry)) => entry,
    Err(_) => LifeLogEntry::Error(line.to_string()),
  };
  Ok((rem, entry))
}
