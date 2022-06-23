pub fn helper_function(x: i32) -> i32 {
  if x == 0 {
    return 0;
  }
  x + 2
}

#[allow(dead_code)]
pub struct S {
  pub x: i32,
}
