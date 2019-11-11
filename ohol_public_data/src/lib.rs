mod parser;

use nom::combinator::all_consuming;
use thiserror::Error;

pub const TARGET: &'static str = "http://publicdata.onehouronelife.com/publicLifeLogData/lifeLog_bigserver2.onehouronelife.com/2019_11November_10_Sunday.txt";

#[derive(Clone, Debug, PartialEq)]
pub struct PlayerId(usize);

#[derive(Clone, Debug, PartialEq)]
pub enum Sex {
  Female,
  Male,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Coords(isize, isize);

#[derive(Clone, Debug)]
pub struct Birth {
  pub unix_time: usize,
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
  pub unix_time: usize,
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

#[derive(Debug, Error)]
pub enum Error {
  #[error("failed to fetch data")]
  Request(#[from] reqwest::Error),
  #[error("failed to parse data")]
  Parsing(String),
}

pub fn get(target: &str) -> Result<LifeLog, Error> {
  let text = reqwest::get(target)?.text()?;
  let parser = all_consuming(parser::life_log);
  parser(&text)
    .or_else(|err| Err(Error::Parsing(format!("{:?}", err))))
    .map(|(_, life_log)| life_log)
}
