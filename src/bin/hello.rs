#![no_std]
#![no_main]

use mushix::api::syscall;
use mushix::entry_point;

entry_point!(main);

fn main(_args: &[&str]) -> usize {
    syscall::write(1, b"Hello, World!\n");
    0
}
