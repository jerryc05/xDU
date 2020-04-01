#![feature(exact_size_is_empty)]
#![feature(const_int_pow)]

use std::fs::{read_dir, symlink_metadata};
use crate::parse_args::parse_args;
use crate::convert_unit::convert_unit;

mod parse_args;
mod xdu;
mod convert_unit;

fn main() {
  let mut config = parse_args();

  while let Some(path) = config.paths.pop() {
    if path.is_dir() {
      for entry in read_dir(path).unwrap() {
        let path_ = entry.unwrap().path();
        if path_.is_dir() {
          config.paths.push(path_);
        } else {
          let (amount, label) = convert_unit(
            symlink_metadata(&path_).unwrap().len());
          println!("{:>7.3} {}: {}",
                   amount, label, path_.to_string_lossy());
        }
      }
    }
  }
}
