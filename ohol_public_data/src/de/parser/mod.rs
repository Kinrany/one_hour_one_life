mod birth;
mod common;
mod death;

use crate::Entry;
use birth::*;
use death::*;
use nom::{branch::alt, combinator::map, IResult};

fn birth_entry(i: &str) -> IResult<&str, Entry> {
  map(birth, Entry::Birth)(i)
}

fn death_entry(i: &str) -> IResult<&str, Entry> {
  map(death, Entry::Death)(i)
}

pub fn entry(i: &str) -> IResult<&str, Entry> {
  alt((birth_entry, death_entry))(i)
}
