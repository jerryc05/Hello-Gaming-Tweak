use std::fs::File;

use crate::tweak_wrapper::Tweakable;

#[allow(dead_code)]
pub(crate) struct FileTweak<'a> {
  pub(crate) path: PathType<'a>,
  pub(crate) action: &'a [FileTweakAction<'a>],
}

#[allow(dead_code)]
pub(crate) enum PathType<'a> {
  File(File),
  Folder(&'a str),
}

#[allow(dead_code)]
pub(crate) enum FileTweakAction<'a> {
  Delete(),
  Modify(File, fn() -> Result<(), &'a str>),
  NewFile(File, &'a str),
  NewFolder(&'a str),
  Rename(File),
}

impl Tweakable for FileTweak<'_> {
  fn is_tweaked(&self) -> bool {
    unimplemented!()
  }

  fn tweak<'a>(&self) -> Result<(), &'a str> {
    unimplemented!()
  }
}