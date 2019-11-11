use ohol_public_data::{url::build_url, Cause::*, LifeLogEntry};
use std::env::args;

fn main() {
  let arg = args().nth(1).expect("No command given");
  let url = build_url("bigserver2", 2019, 01, 30);
  match ohol_public_data::get(&url) {
    Ok(entries) => match arg.as_str() {
      "first_4" => print_first_4(entries.into_iter()),
      "unknown_causes" => print_unknown_causes(entries.into_iter()),
      _ => eprintln!("Unknown command"),
    },
    Err(err) => eprintln!("{}", err),
  };
}

fn print_first_4<E>(entries: E)
where
  E: Iterator<Item = LifeLogEntry> + Sized,
{
  entries.take(4).for_each(|entry| println!("{:?}", entry));
}

fn print_unknown_causes<E>(entries: E)
where
  E: Iterator<Item = LifeLogEntry> + Sized,
{
  unknown_causes(entries)
    .into_iter()
    .for_each(|entry| println!("{:?}", entry));
}

fn unknown_causes<E>(entries: E) -> Vec<String>
where
  E: Iterator<Item = LifeLogEntry> + Sized,
{
  let deaths = entries
    .map(|entry| match entry {
      LifeLogEntry::Death(death) => Some(death),
      LifeLogEntry::Birth(_) => None,
      LifeLogEntry::Error(_) => None,
    })
    .filter(|death| death.is_some())
    .map(|death| death.unwrap());
  let mut other_causes: Vec<String> = deaths
    .map(|death| death.cause)
    .map(|cause| match cause {
      Other(other) => Some(other),
      _ => None,
    })
    .filter(|other| other.is_some())
    .map(|other| other.unwrap())
    .collect();
  other_causes.sort();
  other_causes.dedup();
  other_causes
}
