#![no_std]
#![no_main]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use mushix::{print, println, sys, usr};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    mushix::init(boot_info);
    print!("\x1b[?25h"); // Enable cursor
    loop {
        if let Some(cmd) = option_env!("MUSHIX_CMD") {
            let path: &[&str] = &["~"];
            let prompt = usr::shell::prompt_string(true, path);
            println!("{}{}", prompt, cmd);
            usr::shell::exec(cmd);
            sys::acpi::shutdown();
        } else {
            usr::shell::exec("clear");
            usr::shell::exec("read /ini/banner.txt");
            user_boot();
        }
    }
}

fn user_boot() {
    let script = "/ini/boot.sh";
    if sys::fs::File::open(script).is_some() {
        usr::shell::main(&["shell", script]);
    } else {
        if sys::fs::is_mounted() {
            println!("Could not find '{}'", script);
        } else {
            println!("MFS is not mounted to '/'");
        }
        println!("Running console in diskless mode");
        usr::shell::main(&["shell"]);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        sys::time::sleep(10.0)
    }
}
