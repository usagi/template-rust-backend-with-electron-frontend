use std::{
  ffi::{
    CStr,
    CString
  },
  os::raw::{
    c_char,
    c_ulonglong
  }
};

#[no_mangle]
pub extern fn example_add(
  a: f32,
  b: f32
) -> f32
{
  println!("[from native lib] example_add start");
  println!("[from native lib] a = {}, b = {}", a, b);
  let c = a + b;
  println!("[from native lib] c = {}", c);
  println!("[from native lib] example_add end");
  c
}

/// # Safety
/// Must call `example_concat_free` after used.
#[no_mangle]
pub unsafe extern fn example_concat(
  a: *const c_char,
  b: *const c_char
) -> *mut c_char
{
  println!("[from native lib] example_concat start");
  let sa = CStr::from_ptr(a).to_str().expect("cannot to_str from a");
  let sb = CStr::from_ptr(b).to_str().expect("cannot to_str from b");
  println!("[from native lib] sa={}, sb={}", sa, sb);
  let sr = format!("{}{}", sa, sb);
  println!("[from native lib] sr={}", sr);
  let cr = CString::new(sr).expect("cannot create cr from sr");
  println!("[from native lib] cr={} *=0x{:X}", cr.to_str().expect("cannot to_str from cr"), cr.as_ptr() as c_ulonglong);
  println!("[from native lib] example_concat end");
  cr.into_raw()
}

/// # Safety
/// This is the memory freeing pair for `example_concat`
#[no_mangle]
pub unsafe extern fn example_concat_free(p: *mut c_char)
{
  println!("[from native lib] example_concat_free start");
  println!("[from native lib] *=0x{:X}", p as c_ulonglong);
  if !p.is_null()
  {
    println!("A");
    CString::from_raw(p);
    println!("B");
  }
  println!("C");
  println!("[from native lib] example_concat_free end");
}

#[cfg(test)]
mod tests
{
  use super::*;
  #[test]
  fn add()
  {
    let a = -1.23f32;
    let b = 4.56f32;
    let expected = a + b;
    let actual = example_add(a, b);
    assert_eq!(actual, expected);
  }

  #[test]
  fn concat()
  {
    let a = "su";
    let b = "shi";
    let expected = format!("{}{}", a, b);

    // note: memory image of `str::to_ptr()` is not a null terminated.
    let ca = CString::new(a).unwrap();
    let cb = CString::new(b).unwrap();

    let actual = unsafe { example_concat(ca.as_ptr(), cb.as_ptr()) };
    let actual_str = String::from(unsafe { CStr::from_ptr(actual).to_str().unwrap() });
    unsafe { example_concat_free(actual) };

    assert_eq!(actual_str, expected);
  }
}
