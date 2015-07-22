extern crate hal;
extern crate docopt;

use docopt::Docopt;
use hal::{Model, Layer}

static USAGE: &'static str = "
Usage: 
  hal <input_csv> <output_csv>
";

fn create_model() -> Model{
  Model("L2", "SGD")
}

fn main() {
  let args = Docopt::new(USAGE)
                     .and_then(|dopt| dopt.parse())
                     .unwrap_or_else(|e| e.exit());
  println!("{:?}", args);

  let model = create_model();
  model.info();
}