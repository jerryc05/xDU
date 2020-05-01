#![allow(dead_code)]

#[macro_use]
pub mod my_err;

#[inline]
pub(crate) const fn is_debug() -> bool {
  cfg!(debug_assertions)
}

#[macro_use]
macro_rules! static_assert {
  ($x:expr $(,)?) => {
    const _: [
      ();
      0 - !{ const ASSERT: bool = $x; ASSERT } as usize
    ] = [];
  };
}