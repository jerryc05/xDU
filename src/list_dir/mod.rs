#![allow(unused_imports)]
#![allow(dead_code)]

use std::ffi::OsString;
use std::mem;
use std::ptr;

use num_traits::AsPrimitive;

use crate::utils::my_err::*;

#[cfg(target_os = "windows")]
pub mod win;
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;

#[inline]
pub(crate) fn list_dir() -> Result<u64> {
  #[cfg(target_os = "windows")] {
    win::win()
  }
  #[cfg(target_os = "linux")] {
    linux()
  }
  #[cfg(target_os = "macos")] {
    macos()
  }
  #[cfg(not(any(
  target_os = "windows",
  target_os = "linux",
  target_os = "macos"
  )))] {
    workaround()
  }
}


#[cfg(not(any(
target_os = "windows",
target_os = "linux",
target_os = "macos"
)))]
fn workaround() {
  unimplemented!()
}