#[path = "./tools.rs"]
mod tools;

use crate::tools::S;
use tools::*;

pub fn perform(s: &S) {
  let n = s.x;
  assert!(helper_function(n) % 2 == 1, "should be odd");
}
