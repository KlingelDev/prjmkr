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
