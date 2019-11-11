use nom::{branch::alt, character::complete::char, combinator::map, IResult};

#[derive(Clone, Debug, PartialEq)]
pub enum Sex {
  Female,
  Male,
}

pub fn sex(i: &str) -> IResult<&str, Sex> {
  let parser = alt((char('F'), char('M')));
  map(parser, |chr| match chr {
    'F' => Sex::Female,
    'M' => Sex::Male,
    _ => unreachable!(),
  })(i)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parsing_sex_succeeds() {
    assert_eq!(sex("F"), Ok(("", Sex::Female)));
    assert_eq!(sex("M"), Ok(("", Sex::Male)));
  }
}
