#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

use crate::libc_util::*;

mod file_tweak;
mod libc_util;
mod reg_tweak;
mod tweak_wrapper;
mod tweaks_collection;

#[no_mangle]
pub extern fn mainCRTStartup() {}


#[no_mangle]
pub extern fn main(_argc: isize, _argv: *const *const u8) -> isize {
  p_str("\
* * * * * * * * * * * * * * * * * * * * * * * * * * * *
* ╦ ╦┌─┐┬  ┬  ┌─┐   ╔═╗┌─┐┌┬┐┬┌┐┌┌─┐  ╔╦╗┬ ┬┌─┐┌─┐┬┌─ *
* ╠═╣├┤ │  │  │ │───║ ╦├─┤││││││││ ┬───║ │││├┤ ├─┤├┴┐ *
* ╩ ╩└─┘┴─┘┴─┘└─┘   ╚═╝┴ ┴┴ ┴┴┘└┘└─┘   ╩ └┴┘└─┘┴ ┴┴ ┴ *
* * * * * * * * * * * * * * * * * * * * * * * * * * * *");
  0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}