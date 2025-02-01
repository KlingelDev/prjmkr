#[cfg(test)]
mod tests_createfile {
use prjmkr::createfile::*;

#[test]
fn test_createfile() {
  assert!(create_code() == 0);
}

#[test]
fn test_create_prjfile() {
  let x = PrjFile::new(String::from("test"),
                       PrjFiletypes::CPP,
                       String::from("/test"),
                       String::from("MIT"),
                       String::from("Tester"),
                       2025);

  assert!(x.copyrightyear() == 2025);
}

}  // mod tests_createfile
