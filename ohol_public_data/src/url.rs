use chrono::NaiveDate;

pub fn build_date_str(date: NaiveDate) -> String {
  date.format("%Y_%m%B_%d_%A").to_string()
}

pub fn build_url(server_name: &str, year: i32, month: u32, day: u32) -> String {
  format!(
    "http://publicdata.onehouronelife.com/publicLifeLogData/lifeLog_{}.onehouronelife.com/{}.txt",
    server_name,
    build_date_str(NaiveDate::from_ymd(year, month, day))
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bigserver2_2019_january_30th() {
    let expected = "http://publicdata.onehouronelife.com/publicLifeLogData/lifeLog_bigserver2.onehouronelife.com/2019_01January_30_Wednesday.txt";
    assert_eq!(build_url("bigserver2", 2019, 01, 30), expected);
  }

  #[test]
  fn server15_2019_november_10th() {
    let expected = "http://publicdata.onehouronelife.com/publicLifeLogData/lifeLog_server15.onehouronelife.com/2019_11November_10_Sunday.txt";
    assert_eq!(build_url("server15", 2019, 11, 10), expected);
  }
}
