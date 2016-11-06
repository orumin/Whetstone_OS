#![no_std]
#![feature(asm)]
#![feature(intrinsics)]
#![feature(lang_items)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(missing_copy_implementations)]

extern crate uefi;
extern crate rlibc;

use uefi::SimpleTextOutput;
use uefi::Status;

#[no_mangle]
pub extern "win64" fn efi_main(hdl: uefi::Handle, sys: uefi::SystemTable) -> Status {
    uefi::set_system_table(&sys);
    uefi::get_system_table().console().write("Hello, World!\n\r");

    loop {}
    Status::Success
}

#[no_mangle]
pub fn abort() -> ! {
    loop {}
}

#[no_mangle]
pub fn breakpoint() -> ! {
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {
    loop {}
}
