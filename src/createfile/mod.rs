pub enum PrjFiletypes {
  CPP,
  H,
  PY,
  RS,
  CS,
  LICENCE,
  README
}

/// Traits for PrjFile base file type
pub trait PrjFileTrait {
  fn new(name: String,
         ft: PrjFiletypes,
         path: String,
         cpyrighttemplate: String,
         cpyrightowner: String,
         cpyrightyear: i32) -> Self;

  fn copyrightyear(&self) -> i32;
}

/// Base file type
pub struct PrjFile {
  name: String,
  filetype: PrjFiletypes,
  path: String,
  copyrighttemplate: String,
  copyrightowner: String,
  copyrightyear: i32
}

/// Trait implementation for PrjFile
impl PrjFileTrait for PrjFile {
  fn new(name: String,
         ft: PrjFiletypes,
         path: String,
         cpyrighttemplate: String,
         cpyrightowner: String,
         cpyrightyear: i32) -> Self {
    return Self {
      name: name,
      filetype: ft,
      path: path,
      copyrighttemplate: cpyrighttemplate,
      copyrightowner: cpyrightowner,
      copyrightyear: cpyrightyear
    };
  }

  fn copyrightyear(&self) -> i32 {
    return self.copyrightyear.clone();
  }
}

struct PrjCodeFile {

}

struct PrjHeaderFile {

}

pub fn create_code() -> i8 {
 return 0;
}

pub fn create_header() -> i8 {
 return 0;
}

pub fn get_licence_header() -> i8 {
 return 0;
}

pub fn add_namespace() {

}
