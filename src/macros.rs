#[macro_export]
macro_rules! fmt(($token:expr) => (format!("{:?}", $token)));

#[cfg(test)]
mod test {

  #[test]
  fn test_macro_works_at_all() {
    let _ = fmt!(100);
  }
}
