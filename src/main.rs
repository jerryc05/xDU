#![feature(exact_size_is_empty)]
#![feature(const_int_pow)]

use crate::parse_args::parse_args;
use crate::convert_unit::convert_unit;

mod parse_args;
mod xdu;
mod convert_unit;

fn main() {
  let mut config = parse_args();

  let mut result = vec![];
  while let Some(path) = config.paths.pop() {
    if path.is_dir() {
      for entry in path.read_dir().unwrap() {
        let path_ = entry.unwrap().path();
        if path_.is_dir() {
          config.paths.push(path_);
        } else {
          result.push((
            path_.symlink_metadata().unwrap().len(),
            path_.into_os_string()
          ))
        }
      }
    }
  }

  result.sort_unstable_by(
    |(a, _), (b, _)| a.cmp(b));

  for (len, path) in &result {
    let (amount, label) = convert_unit(*len);
    println!("{:>7.3} {}: {}", amount, label, path.to_string_lossy());
  }
}
