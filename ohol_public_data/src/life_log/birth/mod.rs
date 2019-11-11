pub mod parser;

use super::common::{coords::*, player_id::*, sex::*};

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
