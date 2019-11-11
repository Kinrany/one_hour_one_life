use crate::life_log::{entry::life_log_entry, LifeLog};
use nom::{multi::many0, IResult};

pub fn life_log(i: &str) -> IResult<&str, LifeLog> {
  many0(life_log_entry)(i)
}
