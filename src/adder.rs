pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    assert_eq!(3, add(1, 2));
    assert_eq!(0, add(-1, 1));
    assert_eq!(1, add(0, 1));
  }
}
