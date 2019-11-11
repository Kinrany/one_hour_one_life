use super::{parser, LifeLog};
use anyhow::{anyhow, Result};
use nom::combinator::all_consuming;

pub fn get(target: &str) -> Result<LifeLog> {
  let text = reqwest::get(target)?.text()?;
  let parser = all_consuming(parser::life_log);
  parser(&text)
    .or_else(|err| Err(anyhow!(format!("{:?}", err))))
    .map(|(_, life_log)| life_log)
}
