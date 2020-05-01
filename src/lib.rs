#![feature(const_fn)]
#![feature(const_if_match)]
#![feature(const_int_pow)]
#![feature(with_options)]

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::convert_unit::convert_unit;
use crate::parse_args::parse_args;

pub mod convert_unit;
pub mod list_dir;
pub mod my_err;
pub mod parse_args;
pub mod utils;
pub mod xdu;

const FILE_NAME: &str = "log.log";

pub fn main() {
  let mut config = parse_args();
  println!("Working on it ...");

  let mut result = vec![];
  while let Some(path) = config.paths.pop() {
    if path.is_file() {
      result.push((path.symlink_metadata().unwrap().len(),
                   path.into_os_string()))
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
          eprintln!("Failed to read [{}] (aka [{}]): {}", path.to_string_lossy(),
                    path.canonicalize().unwrap().to_string_lossy(), e);
        }
      }
    }
  }

  result.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

  let file = File::with_options().create(true).write(true).truncate(true)
                                 .open(FILE_NAME).unwrap();
  let mut log = BufWriter::new(file);

  for (len, path) in &result {
    let (amount, label) = convert_unit(*len);

    let precision = 3;
    let whole_len = precision + 4 + 1;

    write!(log, "{:>1$.2$}", amount, whole_len, precision)
      .unwrap_or_else(|_| {});
    print!("{:>1$.2$}", amount, whole_len, precision);

    writeln!(log, " {:>3}: {}", label, path.to_string_lossy())
      .unwrap_or_else(|_| {});
    println!(" {:>3}: {}", label, path.to_string_lossy());
  }

  println!();
  println!("{:-<64}", "");
  println!("Log file saved to [{}]!",
           Path::new(FILE_NAME).canonicalize().unwrap()
                               .to_string_lossy());
  println!("{:-<64}", "");
}
