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

pub use crate::list_dir::win::*;
use crate::utils::my_err::*;

// ref: https://docs.microsoft.com/en-us/windows/win32/api/ioapiset/nf-ioapiset-deviceiocontrol

#[allow(non_snake_case)]
#[inline]
pub fn win_GetFileAttributesExW(lpFileName: LPCWSTR) -> Result<u64> {
  unsafe {
    let mut fInfo: WIN32_FILE_ATTRIBUTE_DATA = mem::zeroed();

    let result: BOOL = GetFileAttributesExW(
      lpFileName, GetFileExInfoStandard,
      &mut fInfo as *mut WIN32_FILE_ATTRIBUTE_DATA as LPVOID);

    if result == 0 {
      Err(my_err_of_str!(format!(
        "Failed to get size of file: [{:?}], error: [{:?}]!",
        str_of_pwchar(lpFileName), str_of_last_err()),  4))
    } else {
      let hi = (fInfo.nFileSizeHigh as u64) <<
        (8 * mem::size_of_val(&fInfo.nFileSizeLow));
      Ok(hi | (fInfo.nFileSizeLow as u64))
    }
  }
}

#[allow(non_snake_case)]
#[inline]
pub fn win_GetFileInformationByHandleEx(lpFileName: LPCWSTR) -> Result<u64> {
  unsafe {
    let hFile: HANDLE = CreateFileW(
      lpFileName, GENERIC_READ, FILE_SHARE_READ,
      ptr::null_mut(), OPEN_EXISTING,
      0, ptr::null_mut());

    if hFile == INVALID_HANDLE_VALUE {
      return Err(my_err_of_str!(format!(
        "Failed to open file: [{:?}], error: [{:?}]!",
        str_of_pwchar(lpFileName), str_of_last_err()), 4));
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
        "Failed to get size of file: [{:?}], error: [{:?}]!",
        str_of_pwchar(lpFileName), str_of_last_err()), 4))
    } else {
      Ok(fInfo.EndOfFile.QuadPart().as_())
    }
  }
}

#[allow(non_snake_case)]
#[inline]
pub fn win_GetFileSizeEx(lpFileName: LPCWSTR) -> Result<u64> {
  unsafe {
    let hFile: HANDLE = CreateFileW(
      lpFileName, GENERIC_READ, FILE_SHARE_READ,
      ptr::null_mut(), OPEN_EXISTING,
      0, ptr::null_mut());

    if hFile == INVALID_HANDLE_VALUE {
      return Err(my_err_of_str!(format!(
        "Failed to open file: [{:?}], error: [{:?}]!",
        str_of_pwchar(lpFileName), str_of_last_err()), 4));
    }

    let mut size: LARGE_INTEGER = mem::zeroed();
    let result: BOOL = GetFileSizeEx(hFile, &mut size);
    CloseHandle(hFile);

    if result == 0 {
      Err(my_err_of_str!(format!(
        "Failed to get size of file: [{:?}], error: [{:?}]!",
        str_of_pwchar(lpFileName), str_of_last_err()),  4))
    } else {
      Ok(size.QuadPart().as_())
    }
  }
}

#[allow(non_snake_case)]
#[inline]
pub fn win_FindFirstFileW(lpFileName: LPCWSTR) -> Result<u64> {
  unsafe {
    let mut fInfo: WIN32_FIND_DATAW = mem::zeroed();
    let hFile: HANDLE = FindFirstFileW(lpFileName, &mut fInfo);

    if hFile == INVALID_HANDLE_VALUE {
      return Err(my_err_of_str!(format!(
        "Failed to open file: [{:?}], error: [{:?}]!",
        str_of_pwchar(lpFileName), str_of_last_err()), 4));
    }
    CloseHandle(hFile);

    let hi = (fInfo.nFileSizeHigh as u64) <<
      (8 * mem::size_of_val(&fInfo.nFileSizeLow));
    Ok(hi | (fInfo.nFileSizeLow as u64))
  }
}

#[allow(non_snake_case)]
#[inline]
pub fn win_FindFirstFileExW_basic(lpFileName: LPCWSTR) -> Result<u64> {
  unsafe {
    let mut fInfo: WIN32_FIND_DATAW = mem::zeroed();
    let hFile: HANDLE = FindFirstFileExW(
      lpFileName, FindExInfoBasic,
      &mut fInfo as *mut WIN32_FIND_DATAW as LPVOID,
      FindExSearchNameMatch, ptr::null_mut(), 0);

    if hFile == INVALID_HANDLE_VALUE {
      return Err(my_err_of_str!(format!(
        "Failed to open file: [{:?}], error: [{:?}]!",
        str_of_pwchar(lpFileName), str_of_last_err()), 4));
    }
    CloseHandle(hFile);

    let hi = (fInfo.nFileSizeHigh as u64) <<
      (8 * mem::size_of_val(&fInfo.nFileSizeLow));
    Ok(hi | (fInfo.nFileSizeLow as u64))
  }
}

#[allow(non_snake_case)]
#[inline]
pub fn win_FindFirstFileExW_basic_large(lpFileName: LPCWSTR) -> Result<u64> {
  unsafe {
    let mut fInfo: WIN32_FIND_DATAW = mem::zeroed();
    let hFile: HANDLE = FindFirstFileExW(
      lpFileName, FindExInfoBasic,
      &mut fInfo as *mut WIN32_FIND_DATAW as LPVOID,
      FindExSearchNameMatch, ptr::null_mut(),
      FIND_FIRST_EX_LARGE_FETCH);

    if hFile == INVALID_HANDLE_VALUE {
      return Err(my_err_of_str!(format!(
        "Failed to open file: [{:?}], error: [{:?}]!",
        str_of_pwchar(lpFileName), str_of_last_err()), 4));
    }
    CloseHandle(hFile);

    let hi = (fInfo.nFileSizeHigh as u64) <<
      (8 * mem::size_of_val(&fInfo.nFileSizeLow));
    Ok(hi | (fInfo.nFileSizeLow as u64))
  }
}

#[allow(non_snake_case)]
#[inline]
pub fn win_FindFirstFileExW_std(lpFileName: LPCWSTR) -> Result<u64> {
  unsafe {
    let mut fInfo: WIN32_FIND_DATAW = mem::zeroed();
    let hFile: HANDLE = FindFirstFileExW(
      lpFileName, FindExInfoStandard,
      &mut fInfo as *mut WIN32_FIND_DATAW as LPVOID,
      FindExSearchNameMatch, ptr::null_mut(),
      FIND_FIRST_EX_LARGE_FETCH);

    if hFile == INVALID_HANDLE_VALUE {
      return Err(my_err_of_str!(format!(
        "Failed to open file: [{:?}], error: [{:?}]!",
        str_of_pwchar(lpFileName), str_of_last_err()), 4));
    }
    CloseHandle(hFile);

    let hi = (fInfo.nFileSizeHigh as u64) <<
      (8 * mem::size_of_val(&fInfo.nFileSizeLow));
    Ok(hi | (fInfo.nFileSizeLow as u64))
  }
}