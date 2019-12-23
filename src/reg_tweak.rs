use std::borrow::Cow;
use std::process::Command;

use crate::reg_tweak::RegPathType::*;
use crate::reg_tweak::RegValData::*;
use crate::tweak_wrapper::Tweakable;

#[allow(dead_code)]
pub(crate) struct RegTweak<'a> {
  pub(crate) path: RegPathType<'a>,
  pub(crate) data: &'a [(&'a str, RegValData<'a>)],
}

#[allow(dead_code)]
pub(crate) enum RegPathType<'a> {
  Regular(&'a str),
  Wildcard(&'a str),
}

impl AsRef<str> for RegPathType<'_> {
  fn as_ref(&self) -> &str {
    match self {
      Regular(s) => s,
      Wildcard(s) => s
    }
  }
}

#[allow(dead_code)]
pub(crate) enum RegValData<'a> {
  /// String
  RegSz(&'a str),
  //  /// Binary
  //  RegBinary(&'a [u8]),
  /// 32-bit
  RegDword(u32),
  /// 64-bit
  RegQword(u64),
  //  /// Multi-String
  //  RegMultiSz(&'a [&'a str]),
  /// Expandable String
  RegExpandSz(&'a str),
}

impl RegValData<'_> {
  fn type_str(&self) -> &str {
    match self {
      RegSz(_) => "REG_SZ",
      RegDword(_) => "REG_DWORD",
      RegQword(_) => "REG_QWORD",
      RegExpandSz(_) => "REG_EXPAND_SZ"
    }
  }

  fn data_str(&self) -> Cow<str> {
    match self {
      RegSz(s) | RegExpandSz(s) => (*s).into(),
      RegDword(d) => format!("0x{:x}", d).into(),
      RegQword(d) => format!("0x{:x}", d).into()
    }
  }
}

impl PartialEq<str> for RegValData<'_> {
  fn eq(&self, s: &str) -> bool {
    if cfg!(debug_assertions) {
      println!("Comparing [{}] w/ [{}] => [{}]!", self.data_str(), s, self.data_str().eq(s));
    }
    self.data_str().eq(s)
  }
}

impl Tweakable for RegTweak<'_> {
  fn is_tweaked(&self) -> bool {
    let mut result = true;

    for data in self.data {
      let cmd =
        Command::new("REG")
          .arg("QUERY")
          .arg(self.path.as_ref())
          .arg("/v")
          .arg(data.0)
          .output();
      if cmd.is_err() {
        eprintln!("Command execution failed!");
        result = false;
      } else {
        let output = cmd.unwrap();
        let stdout_str = unsafe {
          std::str::from_utf8_unchecked(
            output.stdout.as_ref()).trim()
        };
        if cfg!(debug_assertions) {
          println!("{:?}", stdout_str);
        }
        let mut iter = stdout_str.split_ascii_whitespace();
        let parsed_data = iter.nth_back(0);
        if stdout_str.is_empty() || parsed_data.is_none() {
          result = false
        } else {
          let parsed_data = parsed_data.unwrap();
          if !data.1.eq(parsed_data) {
            result = false;
          }
        }
      }
    }
    result
  }

  fn tweak<'a>(&self) -> Result<(), &'a str> {
    for data in self.data {
      let cmd =
        Command::new("REG")
          .arg("ADD")
          .arg(self.path.as_ref())
          .arg("/v")
          .arg(data.0)
          .arg("/t")
          .arg(data.1.type_str())
          .arg("/d")
          .arg(data.1.data_str().as_ref())
          .status();
      if cmd.is_err() {
        return Err("Command execution failed!");
      }
      if !cmd.unwrap().success() {
        return Err("Command execution returned a non-zero value!");
      }
    }
    Ok(())
  }
}