pub(crate) struct Tweak<'a> {
  pub(crate) name: &'a str,
  pub(crate) desc: &'a str,
  pub(crate) content: &'a dyn Tweakable,
}

pub(crate) trait Tweakable {
  fn is_tweaked(&self) -> bool;
  fn tweak<'a>(&self) -> Result<(), &'a str>;
}