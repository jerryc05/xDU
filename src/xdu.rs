use std::path::PathBuf;

pub(crate) const DEFAULT_DEPTH: u8 = 2;

#[derive(Debug)]
pub(crate) struct XduConfig {
  pub(crate) paths: Vec<PathBuf>,
  pub(crate) depth: u8,
}

impl Default for XduConfig {
  fn default() -> Self {
    Self {
      paths: vec![],
      depth: DEFAULT_DEPTH,
    }
  }
}
