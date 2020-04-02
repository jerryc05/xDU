#![feature(exact_size_is_empty)]
#![feature(const_int_pow)]
#![feature(with_options)]

use crate::parse_args::parse_args;
use crate::convert_unit::convert_unit;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
mod parse_args;
mod xdu;
mod convert_unit;

const FILE_NAME: &str = "log.log";

fn main() {
  let mut config = parse_args();

  let mut result = vec![];
  while let Some(path) = config.paths.pop() {
    if path.is_file() {
      result.push((
        path.symlink_metadata().unwrap().len(),
        path.into_os_string()
      ))
    } else {
      match path.read_dir() {
        Ok(dir_read) => {
          for entry in dir_read {
            let path_ = entry.unwrap().path();
            if path_.is_dir() {
              config.paths.push(path_);
            } else if let Ok(meta) = path_.symlink_metadata() {
              result.push((meta.len(), path_.into_os_string()))
            }
          }
        }
        Err(e) => {
          eprintln!("Failed to read [{:?}] (aka [{:?}]): {}",
                    path,
                    path.canonicalize().unwrap(),
                    e);
        }
      }
    }
  }

  result.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

  let file = File::create(FILE_NAME).unwrap();
  let mut log = BufWriter::new(file);

  for (len, path) in &result {
    let (amount, label) = convert_unit(*len);

    let precision = 3;


    write!(log, "{:>1$.2$}", amount, precision + 4, precision)
      .unwrap_or_else(|_| {});
    writeln!(log, " {:>3}: {}", label, path.to_string_lossy())
      .unwrap_or_else(|_| {});
  }

  println!("logs saved to [{}]!",
           Path::new(FILE_NAME).canonicalize().unwrap().to_string_lossy());
}
