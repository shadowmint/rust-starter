#![feature(test)]
extern crate test;

#[macro_use]
mod macros;
mod bench;

pub fn foo() {
  let _ = fmt!("...");
  println!("Hi");
}
