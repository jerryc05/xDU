pub use std::path::Path;

use crate::utils::my_err::*;

#[cfg(target_os = "windows")]
pub mod win;

pub fn std(path: &Path) -> Result<u64> {
  if !path.is_file() {
    Err(my_err_of_str!("Path is not file!"))
  } else {
    unsafe { std_unchecked(path) }
  }
}

/// # Safety
pub unsafe fn std_unchecked(path: &Path) -> Result<u64> {
  path.symlink_metadata()
      .map_err(|err| my_err_of_err!(err,1))
      .map(|m| m.len())
}