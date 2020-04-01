const UNIT_TUPLE: [(u64, &str); 6] = [
  (1024_u64.pow(1), "KiB"),
  (1024_u64.pow(2), "MiB"),
  (1024_u64.pow(3), "GiB"),
  (1024_u64.pow(4), "TiB"),
  (1024_u64.pow(5), "PiB"),
  (1024_u64.pow(6), "EiB")
];

pub(crate) fn convert_unit(bytes: u64) -> (f32, &'static str) {
  for (amount, label) in UNIT_TUPLE.iter().rev() {
    if bytes > *amount {
      return (bytes as f32 / *amount as f32, label);
    }
  }
  (bytes as f32, "  B")
}