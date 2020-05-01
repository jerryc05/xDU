use std::path::{Path, PathBuf};

use crate::xdu::XduConfig;

#[inline]
pub(crate) fn parse_args() -> XduConfig {
  let mut args = std::env::args_os();
  let mut config = XduConfig::default();
  let mut depth_defined = false;

  if args.len() <= 1 {
    config.paths.push(Path::new(&args.next().unwrap())
      .parent().unwrap().to_path_buf());
  } else {
    args.next();  // Ignore first argv (executable file path)

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
            panic!("Failed to parse [{}] to u8!", depth.to_string_lossy()));
          depth_defined = true;
        }
      } else {
        let path = PathBuf::from(arg);

        /* Validate path string */ {
          if !path.is_dir() && !path.is_file() {
            panic!("The input is not a directory or a file: [{}]!",
                   path.to_string_lossy());
          }
        }

        config.paths.push(path);
      }
    }
  }
  config
}
