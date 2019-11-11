pub mod birth;
mod common;
pub mod death;
pub mod entry;
pub mod get;
pub mod parser;

pub const TARGET: &'static str = "http://publicdata.onehouronelife.com/publicLifeLogData/lifeLog_bigserver2.onehouronelife.com/2019_11November_10_Sunday.txt";

pub type LifeLog = Vec<entry::LifeLogEntry>;
