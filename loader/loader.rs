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

#[allow(unreachable_code)]
#[no_mangle]
pub extern "win64" fn efi_main(hdl: uefi::Handle, sys: uefi::SystemTable) -> uefi::Status {
    uefi::initialize_lib(&hdl, &sys);
    uefi::get_system_table().console().write("Hello, World!\n\r");

    let mut memory_map_size: usize = 0;
    let mut map_key: usize = 0;
    let mut descriptor_size: usize = 0;
    let mut descriptor_version: u32 = 0;

    let memory_map = uefi::lib_memory_map(&mut memory_map_size, &mut map_key, &mut descriptor_size, &mut descriptor_version);

    loop {}
    uefi::Status::Success
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
#[no_mangle]
pub extern fn rust_eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(_msg: core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    loop {}
}
