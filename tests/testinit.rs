#[cfg(test)]
mod tests_actors {
use prjmkr::init::run;

#[test]
fn test_init() {
  let a: Vec<String> = vec![String::from("-test")];
  assert!(run(a) == 0);
}

}
