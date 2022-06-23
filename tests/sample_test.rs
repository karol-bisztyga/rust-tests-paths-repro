mod decomposed_test1;
mod decomposed_test2;
#[path = "./tools.rs"]
mod tools;

use decomposed_test1::perform as p1;
use decomposed_test2::perform as p2;
use tools::*;

#[test]
fn test1() {
  let mut s: S = S { x: 2 };
  p1(&s);
  s.x = 1;
  p2(&s);
  assert!(helper_function(0) == 0, "should be 0");
}
