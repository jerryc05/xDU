const UNIT_TUPLE: [(u64, &str); 6] = {
  let mut i = 0_u32;

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
    (kib, "KiB"),
    (mib, "MiB"),
    (gib, "GiB"),
    (tib, "TiB"),
    (pib, "PiB"),
    (eib, "EiB")
  ]
};

pub(crate) fn convert_unit(bytes: u64) -> (f32, &'static str) {
  for (amount, label) in UNIT_TUPLE.iter().rev() {
    if bytes > *amount {
      return (bytes as f32 / *amount as f32, label);
    }
  }
  (bytes as f32, "B")
}