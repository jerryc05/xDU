#![allow(unused_imports)]

use criterion::*;

use xdu::list_dir::*;

fn bench_list_dir(c: &mut Criterion) {
  c.bench_function(
    "list_dir", |b| b.iter(|| {
      unimplemented!()
    }));
}

criterion_main!(benches);
criterion_group!(benches, bench_list_dir);