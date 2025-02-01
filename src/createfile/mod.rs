/* Copyright 2025 Karl Ruskowski
 *
 * Permission is  hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the “Software”), to deal
 * in the Software without restriction,  including without limitation the rights
 * to use, copy, modify, merge,   publish,  distribute, sublicense,  and/or sell
 * copies   of  the Software,  and   to permit persons to whom   the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE  IS PROVIDED “AS IS”,  WITHOUT  WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED,  INCLUDING BUT NOT LIMITED  TO  THE WARRANTIES   OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE  AND NONINFRINGEMENT. IN NO  EVENT SHALL THE
 * AUTHORS  OR  COPYRIGHT HOLDERS  BE  LIABLE  FOR  ANY CLAIM, DAMAGES  OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 ******************************************************************************/

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
