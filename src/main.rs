#![feature(exact_size_is_empty)]

use std::fs::{read_dir, symlink_metadata};

mod parse_args;
mod xdu;

fn main() {
  let mut config = parse_args::parse();

  while let Some(path) = config.paths.pop() {
    if path.is_dir() {
      for entry in read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path_ = entry.path();
        if path_.is_dir() {
          config.paths.push(path_);
        } else {
          println!("{}: \t{}",
                   path_.to_string_lossy(),
                   symlink_metadata(&path_).unwrap().len());
        }
      }
    }
  }
}
