const UNITS: [(u64, &str); 7] = {
  let mut i = 0;

  let __b = i as u64;
  let kib = {
    1024_u64.pow({
      i += 1;
      i
    })
  };
  let mib = {
    1024_u64.pow({
      i += 1;
      i
    })
  };
  let gib = {
    1024_u64.pow({
      i += 1;
      i
    })
  };
  let tib = {
    1024_u64.pow({
      i += 1;
      i
    })
  };
  let pib = {
    1024_u64.pow({
      i += 1;
      i
    })
  };
  let eib = {
    1024_u64.pow({
      i += 1;
      i
    })
  };

  [
    (__b, "B"),
    (kib, "KiB"),
    (mib, "MiB"),
    (gib, "GiB"),
    (tib, "TiB"),
    (pib, "PiB"),
    (eib, "EiB")
  ]
};

pub(crate) const fn convert_unit(bytes: u64) -> (f32, &'static str) {
  let mut i = 0;
  if bytes < UNITS[i + 1].0 {
    return (bytes as f32, UNITS[i].1);
  }

  i += 1;
  if bytes < UNITS[i + 1].0 {
    return (bytes as f32 / UNITS[i].0 as f32, UNITS[i].1);
  }

  i += 1;
  if bytes < UNITS[i + 1].0 {
    return (bytes as f32 / UNITS[i].0 as f32, UNITS[i].1);
  }

  i += 1;
  if bytes < UNITS[i + 1].0 {
    return (bytes as f32 / UNITS[i].0 as f32, UNITS[i].1);
  }

  i += 1;
  if bytes < UNITS[i + 1].0 {
    return (bytes as f32 / UNITS[i].0 as f32, UNITS[i].1);
  }

  i += 1;
  if bytes < UNITS[i + 1].0 {
    return (bytes as f32 / UNITS[i].0 as f32, UNITS[i].1);
  }

  i += 1;
  (bytes as f32 / UNITS[i].0 as f32, UNITS[i].1)
}

#[test]
fn test_convert_unit() {
  assert_eq!((0., "B"), convert_unit(0));
  assert_eq!((1., "B"), convert_unit(1));
  assert_eq!((1023., "B"), convert_unit(1023));
  assert_eq!((1., "KiB"), convert_unit(1024));
  assert_eq!((1.5, "KiB"), convert_unit((1024 as f32 * 1.5) as u64));
  assert_eq!((1023., "KiB"), convert_unit(1023*1024));
  assert_eq!((1., "MiB"), convert_unit(1024*1024));
}