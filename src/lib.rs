#[macro_use]
mod macros;

mod foo;

pub use foo::Bar;

pub fn foo() {
  let _ = fmt!("...");
  println!("Hi");
}
