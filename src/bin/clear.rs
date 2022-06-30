#![no_std]
#![no_main]

use mushix::api::syscall;
use mushix::entry_point;

entry_point!(main);

fn main(_args: &[&str]) -> usize {
    syscall::write(1, b"\x1b[2J\x1b[1;1H"); // Clear screen and move cursor to top
    0
}
