pub fn example_add(a: f32, b: f32) -> f32 {
 println!("[from native lib] example_add start");
 println!("[from native lib] a = {}, b = {}", a, b);
 let c = a + b;
 println!("[from native lib] c = {}", c);
 println!("[from native lib] example_add end");
 c
}

pub fn example_concat(a: &str, b: &str) -> String {
 println!("[from native lib] example_concat start");
 println!("[from native lib] a={}, b={}", a, b);
 let r = format!("{}{}", a, b);
 println!("[from native lib] r={}", r);
 println!("[from native lib] example_concat end");
 r
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn add() {
  let a = -1.23f32;
  let b = 4.56f32;
  let expected = a + b;
  let actual = example_add(a, b);
  assert_eq!(actual, expected);
 }

 #[test]
 fn concat() {
  let a = "su";
  let b = "shi";
  let expected = format!("{}{}", a, b);

  let actual = example_concat(a, b);

  assert_eq!(actual, expected);
 }
}
