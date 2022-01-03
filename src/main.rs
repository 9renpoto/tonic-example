fn main() {
    println!("Hello, world!");
}

pub fn prints_and_returns_10(a: i32) -> i32 {
  //{}という値を得た
  println!("I got the value {}", a);
  10
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn this_test_will_pass() {
      let value = prints_and_returns_10(4);
      assert_eq!(10, value);
  }
}
