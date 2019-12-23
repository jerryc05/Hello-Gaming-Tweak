use tweaks_collection::*;
use util::*;

use crate::tweak_wrapper::Tweak;
use crate::util::Counter;

mod file_tweak;
mod reg_tweak;
mod tweak_wrapper;
mod tweaks_collection;
mod util;

fn main() {
  println!("\
* * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
*   ╦ ╦┌─┐┬  ┬  ┌─┐   ╔═╗┌─┐┌┬┐┬┌┐┌┌─┐  ╔╦╗┬ ┬┌─┐┌─┐┬┌─   *
*   ╠═╣├┤ │  │  │ │───║ ╦├─┤││││││││ ┬───║ │││├┤ ├─┤├┴┐   *
*   ╩ ╩└─┘┴─┘┴─┘└─┘   ╚═╝┴ ┴┴ ┴┴┘└┘└─┘   ╩ └┴┘└─┘┴ ┴┴ ┴   *
*   v{:52                                               } *
* * * * * * * * * * * * * * * * * * * * * * * * * * * * * *",
           env!("CARGO_PKG_VERSION"));
  let mut counter = Counter(0);
  regular_tweaks(&mut counter);
}

#[inline]
fn regular_tweaks(counter: &mut Counter) {
  process_tweak(DISABLE_MELTDOWN_SPECTRE, counter);
  process_tweak(DISABLE_NAGLE_S_ALGORITHM, counter);
  process_tweak(DISABLE_NETWORK_THROTTLING, counter);
  process_tweak(IMPROVE_SYSTEM_RESPONSIVENESS, counter);
  process_tweak(IMPROVE_GPU_AND_PRIORITIES, counter);
}

#[inline]
fn process_tweak(tweak: Tweak, counter: &mut Counter) {
  println!("-----------------------------------------------------------");
  println!("\
#{:2}:
  NAME:    {}
  PATCHED: {}
  DESC: {}",
           counter.next(),
           tweak.name,
           if tweak.content.is_tweaked() { CHECK_MARK } else { CROSS_MARK },
           tweak.desc);
}