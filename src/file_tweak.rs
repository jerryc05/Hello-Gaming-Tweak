pub struct FileTweak<'a> {
  pub path: PathType<'a>,
  pub action: &'a [FileTweakAction<'a>],
}

pub enum PathType<'a> {
  File(&'a str),
  Folder(&'a str)
}

pub enum FileTweakAction<'a> {
  Delete(),
  Modify(&'a str),
  NewFile(&'a str, &'a str),
  Rename(&'a str),
}