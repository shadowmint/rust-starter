use module1;
use module2;
use module3;

pub fn doit() {
  println!("module4");
  module1::blah::doit();
  module2::blah::doit();
  module3::blah::doit();
}
