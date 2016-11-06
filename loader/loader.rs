#![no_std]
#![no_main]
#![feature(asm)]
#![feature(intrinsics)]
#![feature(lang_items)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(missing_copy_implementations)]

extern crate uefi;

use uefi::SimpleTextOutput;

#[no_mangle]
#[no_stack_check]
pub extern "C" fn efi_main(hdl: uefi::Handle, sys: uefi::SystemTable) {
    uefi::set_system_table(&sys);
    uefi::get_system_table().console().write("Hello, World!\n\r");

    loop {}
}

#[no_mangle]
pub fn abort() -> ! {
    loop {}
}

#[no_mangle]
pub fn breakpoint() -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}
