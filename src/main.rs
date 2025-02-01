use std::env;

use prjmkr::init::run;

fn main() {
  let args: Vec<String> = env::args().collect();
  run(args);
}
