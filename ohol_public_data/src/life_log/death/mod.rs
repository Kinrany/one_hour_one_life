pub mod parser;

use super::common::{coords::*, player_id::*, sex::*};

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
