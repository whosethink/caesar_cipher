mod args;
mod cipher;

use clap::{load_yaml, App};
use crate::args::Args;
use crate::cipher::Cipher;
use std::io::Write;

fn main() {
  let args = Args::from_matches(&App::from(load_yaml!("args.yaml")).get_matches());
  let cipher = Cipher::new(args.get_shift());
  let mut result = cipher.cipher_str(&args.get_text());
  result.push('\n');
  let _ = std::io::stdout().write_all(result.as_bytes());
}