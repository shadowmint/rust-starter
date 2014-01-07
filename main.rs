// Callback function!
extern "C" fn callback(a:i32) {
  println!("Got a callback with {0}", a);
}

// Bind to the test.a static library
#[link(name = "test", kind = "static")]
extern {
  fn bind(cb: extern "C" fn(i32));
  fn invoke();
}

fn main() {
  unsafe { bind(callback) };
  unsafe { invoke() };
  println!("Done!");
}
