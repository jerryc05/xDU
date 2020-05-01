#![allow(unused_imports)]
#![allow(dead_code)]

use std::ffi::OsString;
use std::mem;
use std::ptr;

use num_traits::AsPrimitive;
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::*;
#[cfg(target_os = "windows")]
use winapi::um::errhandlingapi::*;
#[cfg(target_os = "windows")]
use winapi::um::fileapi::*;
#[cfg(target_os = "windows")]
use winapi::um::handleapi::*;
#[cfg(target_os = "windows")]
use winapi::um::minwinbase::*;
#[cfg(target_os = "windows")]
use winapi::um::winbase::*;
#[cfg(target_os = "windows")]
use winapi::um::winnt::*;

use crate::utils::my_err::*;

#[inline]
pub(crate) fn list_dir() {
  #[cfg(target_os = "windows")] {
    win()
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

// ref: https://docs.microsoft.com/en-us/windows/win32/api/ioapiset/nf-ioapiset-deviceiocontrol
#[inline]
#[cfg(target_os = "windows")]
fn win() {
  unimplemented!()
}

#[allow(non_snake_case)]
#[inline]
#[cfg(target_os = "windows")]
pub fn win_GetFileSizeEx(lpFileName: LPCWSTR) -> Result<u64> {
  unsafe {
    let hFile: HANDLE = CreateFileW(
      lpFileName, GENERIC_READ, FILE_SHARE_READ,
      ptr::null_mut(), OPEN_EXISTING,
      0, ptr::null_mut());

    if hFile == INVALID_HANDLE_VALUE {
      return Err(my_err_of_str!(format!(
        "Failed to open file: [{:?}], error: [{}]!",
        str_of_pwchar(lpFileName), GetLastError()), 4));
    }

    let mut size: LARGE_INTEGER = mem::zeroed();
    let result: BOOL = GetFileSizeEx(hFile, &mut size);
    CloseHandle(hFile);

    if result == 0 {
      Err(my_err_of_str!(format!(
        "Failed to get size of file: [{:?}], error: [{}]!",
        str_of_pwchar(lpFileName), GetLastError()),  4))
    } else {
      Ok(size.QuadPart().as_())
    }
  }
}

#[allow(non_snake_case)]
#[inline]
#[cfg(target_os = "windows")]
pub fn win_GetFileAttributesExW(lpFileName: LPCWSTR) -> Result<u64> {
  unsafe {
    let mut fInfo: WIN32_FILE_ATTRIBUTE_DATA = mem::zeroed();

    let result: BOOL = GetFileAttributesExW(
      lpFileName, GetFileExInfoStandard,
      &mut fInfo as *mut WIN32_FILE_ATTRIBUTE_DATA as LPVOID);

    if result == 0 {
      Err(my_err_of_str!(format!(
        "Failed to get size of file: [{:?}], error: [{}]!",
        str_of_pwchar(lpFileName), GetLastError()),  4))
    } else {
      let hi = (fInfo.nFileSizeHigh as u64) <<
        (8 * mem::size_of_val(&fInfo.nFileSizeLow));
      Ok(hi | (fInfo.nFileSizeLow as u64))
    }
  }
}

#[allow(non_snake_case)]
#[inline]
#[cfg(target_os = "windows")]
pub fn win_PrvGetFileInformationByHandleEx(lpFileName: LPCWSTR) -> Result<u64> {
  unsafe {
    let hFile: HANDLE = CreateFileW(
      lpFileName, GENERIC_READ, FILE_SHARE_READ,
      ptr::null_mut(), OPEN_EXISTING,
      0, ptr::null_mut());

    if hFile == INVALID_HANDLE_VALUE {
      return Err(my_err_of_str!(format!(
        "Failed to open file: [{:?}], error: [{}]!",
        str_of_pwchar(lpFileName), GetLastError()), 4));
    }

    let mut fInfo: FILE_STANDARD_INFO = mem::zeroed();

    static_assert!(mem::size_of::<FILE_STANDARD_INFO>()
      <=(u32::max_value() as usize));


    let result: BOOL = GetFileInformationByHandleEx(
      hFile, FileStandardInfo,
      &mut fInfo as *mut FILE_STANDARD_INFO as LPVOID,
      mem::size_of::<FILE_STANDARD_INFO>() as u32);
    CloseHandle(hFile);

    if result == 0 {
      Err(my_err_of_str!(format!(
        "Failed to get size of file: [{:?}], error: [{}]!",
        str_of_pwchar(lpFileName), GetLastError()), 4))
    } else {
      Ok(fInfo.EndOfFile.QuadPart().as_())
    }
  }
}

#[allow(non_snake_case)]
#[inline]
#[cfg(target_os = "windows")]
pub fn win_FindFirstFile(lpFileName: LPCWSTR) -> Result<u64> {
  unsafe {
    let mut fInfo: WIN32_FIND_DATAW = mem::zeroed();
    let hFile: HANDLE = FindFirstFileW(lpFileName, &mut fInfo);

    if hFile == INVALID_HANDLE_VALUE {
      return Err(my_err_of_str!(format!(
        "Failed to open file: [{:?}], error: [{}]!",
        str_of_pwchar(lpFileName), GetLastError()), 4));
    }
    CloseHandle(hFile);

    let hi = (fInfo.nFileSizeHigh as u64) <<
      (8 * mem::size_of_val(&fInfo.nFileSizeLow));
    Ok(hi | (fInfo.nFileSizeLow as u64))
  }
}


#[inline]
#[cfg(target_os = "windows")]
fn str_of_pwchar(ptr: LPCWSTR) -> Result<String> {
  if ptr.is_null() {
    return Err(my_err_of_str!("Null pointer!", 1));
  }
  unsafe {
    use std::slice;
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    let len = {
      let mut len = 0;
      loop {
        if *ptr.offset(len) == 0 {
          break;
        }
        len += 1;
      }
      len + 1
    }; // todo potential optimization
    let slice = slice::from_raw_parts(ptr, len as usize);
    let os_str: OsString = OsString::from_wide(slice);

    Ok(os_str.to_string_lossy().into_owned())
  }
}

// ref: https://github.com/BurntSushi/walkdir/issues/120
// ref: https://github.com/romkatv/gitstatus/blob/master/docs/listdir.md
#[cfg(target_os = "linux")]
fn linux() {
  unimplemented!()
}

#[cfg(target_os = "macos")]
fn macos() {
  unimplemented!()
}

#[cfg(not(any(
target_os = "windows",
target_os = "linux",
target_os = "macos"
)))]
fn workaround() {
  unimplemented!()
}