use crate::xdu::{DEFAULT_DEPTH, XduConfig};
use std::path::PathBuf;

#[inline]
pub(crate) fn parse() -> XduConfig {
  let mut args = std::env::args_os();

  if args.len() <= 1 {
    println!("
NAME
\txdu -- Display Disk Usage - eXperimental version by @jerryc05

SYNOPSIS
\txdu [-d depth] directory [dir2 dir3...]
DESCRIPTION
\tThis xdu tool is a simplified version of *ix's `du` command.
Fore more info, please refer to `man du`.
\tThe options are as follows:

\t-d `depth`
\t\tPrint only `depth` level of directory. Maximum value for
\t\t`depth` is 2^8. Default value is {}.
", DEFAULT_DEPTH);
    panic!("No argument specified!");
  }

  let mut config = XduConfig::default();
  let mut depth_defined = false;

  while let Some(arg) = args.next() {
    if arg == "-d" {
      let depth;

      /* Error handling */ {
        if depth_defined {
          panic!("Duplicate definition of \"-d\"!");
        }
        depth = args.next().expect("No input for `depth` after \"-d\"!");
      }

      /* Parse depth variable */ {
        config.depth = depth.to_string_lossy().parse().unwrap_or_else(|_|
          panic!("Failed to parse [{:?}] to u8!", depth));
        depth_defined = true;
      }
    } else {
      let path = PathBuf::from(arg);

      /* Validate path string */ {
        if !path.is_dir() && !path.is_file() {
          panic!("The input is not a directory or a file: [{:?}]!", path);
        }
      }

      config.paths.push(path);
    }
  }
  return config;
}
