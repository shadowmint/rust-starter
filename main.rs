use std::libc::{c_int};

// Bind to the test.a static library
#[link(name = "test", kind = "static")]
extern {
  fn check(a:c_int, b:c_int) -> c_int;
}

fn main() {
  let x = unsafe  { check(10, 20) };
  println!("{} was the output", x);
}
