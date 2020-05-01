#![allow(unused_imports)]

use criterion::*;

use xdu::file_size::*;
#[cfg(target_os = "windows")]
use xdu::file_size::win::*;

#[cfg(target_os = "windows")]
fn bench_file_size_win(c: &mut Criterion) {
  let file_name = file!();
  assert!(Path::new(file_name).exists());

  let file_name_w = str_to_pwchar(file_name);
  let mut group = c.benchmark_group("bench_file_size_win");

  group.bench_function(
    "win_GetFileSizeEx",
    |b| b.iter(
      || win_GetFileSizeEx(file_name_w.as_ptr())));

  group.bench_function(
    "win_GetFileAttributesExW",
    |b| b.iter(
      || win_GetFileAttributesExW(file_name_w.as_ptr())));

  group.bench_function(
    "win_GetFileInformationByHandleEx",
    |b| b.iter(
      || win_GetFileInformationByHandleEx(file_name_w.as_ptr())));

  group.bench_function(
    "win_FindFirstFileW",
    |b| b.iter(
      || win_FindFirstFileW(file_name_w.as_ptr())));

  group.bench_function(
    "win_FindFirstFileExW_basic",
    |b| b.iter(
      || win_FindFirstFileExW_basic(file_name_w.as_ptr())));

  group.bench_function(
    "win_FindFirstFileExW_std",
    |b| b.iter(
      || win_FindFirstFileExW_std(file_name_w.as_ptr())));

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