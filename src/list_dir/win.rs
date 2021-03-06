#![allow(unused_imports)]
#![allow(dead_code)]

use core::{iter, mem};
use core::ptr;
use core::slice;
use std::ffi::OsString;
use std::os::windows::ffi::*;

use num_traits::AsPrimitive;
use winapi::shared::minwindef::*;
use winapi::um::errhandlingapi::*;
use winapi::um::fileapi::*;
use winapi::um::handleapi::*;
use winapi::um::minwinbase::*;
use winapi::um::winbase::*;
use winapi::um::winnt::*;

use crate::utils::my_err::*;

#[inline]
pub(crate) fn win() -> Result<u64> {
  unimplemented!()
}

#[inline]
pub fn str_to_pwchar(s: &str) -> Vec<u16> {
  OsString::from(s).encode_wide().chain(iter::once(0)).collect()
}

#[inline]
pub fn str_of_pwchar(ptr: LPCWSTR) -> Result<String> {
  if ptr.is_null() {
    return Err(my_err_of_str!("Null pointer!", 1));
  }
  unsafe {
    let len = {
      let mut len = 0;
      loop {
        if *ptr.offset(len) == 0 {
          break;
        }
        len += 1;
      }
      len
    }; // todo potential optimization
    let slice = slice::from_raw_parts(ptr, len as usize);
    let os_str: OsString = OsString::from_wide(slice);

    Ok(os_str.to_string_lossy().into_owned())
  }
}

#[inline]
pub(crate) fn str_of_last_err() -> Result<String> {
  unsafe {
    let err_code: DWORD = GetLastError();
    let mut err_str: LPWSTR = ptr::null_mut();

    if FormatMessageW(
      FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_ALLOCATE_BUFFER
        | FORMAT_MESSAGE_IGNORE_INSERTS, ptr::null(), err_code,
      MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT) as u32,
      &mut err_str as *mut LPWSTR as LPWSTR,
      0, ptr::null_mut(),
    ) == 0 {
      Err(my_err_of_str!(format!(
        "Failed to get string of error code: [{}]!",
        err_code), 4))
    } else {
      let result = str_of_pwchar(err_str);
      LocalFree(err_str as LPVOID);
      result
    }
  }
}