use crate::reg_tweak::RegTweak;

pub struct Tweak<'a> {
  pub t_type: TweakType<'a>,
  pub desc:&'a str
}

pub enum TweakType<'a> {
  Reg(RegTweak<'a>)
}