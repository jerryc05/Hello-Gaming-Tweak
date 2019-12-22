extern crate  alloc;
use alloc::string::ToString;

extern {
  fn printf(fmt: *const u8, ...) -> i32;
}

pub fn p_str(s: &str) -> i32 {
  if !s.ends_with('\0') {
    if cfg!(debug_assertions) {
      panic!("C String not null terminated!")
    }
    let mut s = s.to_string();
    s.push('\0');
  }
  unsafe {
    p_str_unchecked(s)
  }
}

#[inline]
pub unsafe fn p_str_unchecked(s: &str) -> i32 {
  printf(s.as_ptr())
}