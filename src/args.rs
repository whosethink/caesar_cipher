use clap::ArgMatches;
use std::ops::Rem;

const SHIFT_MOD: i8 = 26;

#[derive(Debug)]
pub struct Args {
  shift: i8,
  text: String,
}

impl Args {
  pub fn from_matches(matches: &ArgMatches) -> Args {
    let shift = matches.value_of("shift")
      .unwrap()
      .parse::<i8>()
      .unwrap()
      .rem(SHIFT_MOD);
    let text = matches.value_of("text")
      .unwrap().to_string();
    Args {
      shift,
      text,
    }
  }

  pub fn get_shift(&self) -> i8 {
    return self.shift;
  }

  pub fn get_text(&self) -> String {
    return (*self.text).to_string();
  }
}