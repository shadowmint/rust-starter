extern crate foo;

use module3::module4;
use foo::foo as foo_;
use foo::Bar;

#[macro_use]
mod macros;

mod module1;
mod module2;
mod module3;

fn main() {
  module4::blah::doit();
  foo_();
  let _ = Bar;
}
