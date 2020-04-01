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
    if path.is_file() {
      result.push((
        path.symlink_metadata().unwrap().len(),
        path.into_os_string()
      ))
    } else if let Ok(dir_read) = path.read_dir() {
      for entry in dir_read {
        let path_ = entry.unwrap().path();
        if path_.is_dir() {
          config.paths.push(path_);
        } else if let Ok(meta) = path_.symlink_metadata() {
          result.push((meta.len(), path_.into_os_string()))
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
