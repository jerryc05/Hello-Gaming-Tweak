pub(crate) const CROSS_MARK: &str = "❌";
pub(crate) const CHECK_MARK: &str = "✅";

pub(crate) struct Counter(pub(crate) u8);

impl Counter {
  pub(crate) fn next(&mut self) -> u8 {
    self.0 += 1;
    self.0
  }
}