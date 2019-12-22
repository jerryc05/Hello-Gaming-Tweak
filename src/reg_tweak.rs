pub struct RegTweak<'a> {
  pub path: &'a str,
  pub content: &'a [(&'a str, RegValData<'a>)],
}

pub enum RegValData<'a> {
  /// String
  RegSz(&'a str),
  /// Binary
  RegBinary(&'a [u8]),
  /// 32-bit
  RegDword(u32),
  /// 64-bit
  RegQword(u64),
  /// Multi-String
  RegMultiSz(&'a [&'a str]),
  /// Expandable String
  RegExpandSz(&'a str),
}