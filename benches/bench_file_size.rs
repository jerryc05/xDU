#![allow(unused_imports)]

use criterion::*;

use xdu::file_size::*;
#[cfg(target_os = "windows")]
use xdu::file_size::win::*;

#[cfg(target_os = "windows")]
fn bench_file_size_win(c: &mut Criterion) {
  let file_name = file!();
  assert!(Path::new(file_name).exists());

  let mut group = c.benchmark_group("bench_file_size_win");

  macro_rules! wchar {
      () => {str_to_pwchar(file_name).as_ptr();};
  }

  group.bench_function(
    "win_GetFileSizeEx",
    |b| b.iter(
      || win_GetFileSizeEx(wchar!())));

  group.bench_function(
    "win_GetFileAttributesExW",
    |b| b.iter(
      || win_GetFileAttributesExW(wchar!())));

  group.bench_function(
    "win_GetFileInformationByHandleEx",
    |b| b.iter(
      || win_GetFileInformationByHandleEx(wchar!())));

  group.bench_function(
    "win_FindFirstFileW",
    |b| b.iter(
      || win_FindFirstFileW(wchar!())));

  group.bench_function(
    "win_FindFirstFileExW_basic",
    |b| b.iter(
      || win_FindFirstFileExW_basic(wchar!())));

  group.bench_function(
    "win_FindFirstFileExW_basic_large",
    |b| b.iter(
      || win_FindFirstFileExW_basic_large(wchar!())));

  group.bench_function(
    "win_FindFirstFileExW_std",
    |b| b.iter(
      || win_FindFirstFileExW_std(wchar!())));

  group.bench_function(
    "std",
    |b| b.iter(
      || std(Path::new(file_name))));

  group.bench_function(
    "std_unchecked",
    |b| b.iter(
      || unsafe { std_unchecked(Path::new(file_name)) }));
}

criterion_main!(benches);

#[cfg(target_os = "windows")]
criterion_group!(benches, bench_file_size_win);