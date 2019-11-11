mod de;

use reqwest::Result;

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
pub enum Entry {
  Birth(Birth),
  Death(Death),
}

pub fn get() -> Result<Vec<Entry>> {
  let result = reqwest::get(TARGET)?
    .text()?
    .lines()
    .map(|line| de::parser::entry(line).unwrap().1)
    .collect();
  Ok(result)
}
