#![no_std]
#![no_main]

use mushix::api::syscall;
use mushix::entry_point;

entry_point!(main);

fn main(args: &[&str]) -> usize {
    let n = args.len();
    for i in 1..n {
        syscall::write(1, args[i].as_bytes());
        if i < n - 1 {
            syscall::write(1, b" ");
        }
    }
    syscall::write(1, b"\n");
    0
}
