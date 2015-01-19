extern crate foo;

use module3::module4;
use foo::foo;

mod module1;
mod module2;
mod module3;

fn main() {
    module4::blah::doit();
    foo();
}
