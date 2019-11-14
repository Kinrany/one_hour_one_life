mod parser;
pub mod url;

use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use nom::combinator::all_consuming;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct PlayerId(usize);

#[derive(Clone, Debug, PartialEq)]
pub enum Sex {
  Female,
  Male,
}

#[derive(Clone, PartialEq)]
pub struct Coords(isize, isize);

impl fmt::Debug for Coords {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.0, self.1)
  }
}

#[derive(Clone, Debug)]
pub struct Birth {
  pub unix_time: NaiveDateTime,
  pub player_id: PlayerId,
  pub email_hash: String,
  pub sex: Sex,
  pub coords: Coords,
  pub parent: Option<PlayerId>,
  pub pop: usize,
  pub chain: usize,
}

#[derive(Clone, Debug)]
pub enum Cause {
  Disconnect,
  Hunger,
  Killer(PlayerId),
  OldAge,
  Other(String),
}

#[derive(Clone, Debug)]
pub struct Death {
  pub unix_time: NaiveDateTime,
  pub player_id: PlayerId,
  pub email_hash: String,
  pub age: f32,
  pub sex: Sex,
  pub coords: Coords,
  pub cause: Cause,
  pub pop: usize,
}

#[derive(Clone, Debug)]
pub enum LifeLogEntry {
  Birth(Birth),
  Death(Death),
  Error(String),
}

pub type LifeLog = Vec<LifeLogEntry>;

pub fn get(target: &str) -> Result<LifeLog> {
  let text = reqwest::get(target)?.text()?;
  let parser = all_consuming(parser::life_log);
  parser(&text)
    .or_else(|err| Err(anyhow!(format!("{:?}", err))))
    .map(|(_, life_log)| life_log)
}
