#[cfg(test)]
mod benchmark {

  use test::Bencher;

  #[bench]
  fn bench_cmp_1(b: &mut Bencher) {
    b.iter(|| {
      let _ = format!("Hello World");
    });
  }

  #[bench]
  fn bench_cmp_2(b: &mut Bencher) {
    b.iter(|| {
      let _ = format!("Hello World 100000");
    });
  }
}
