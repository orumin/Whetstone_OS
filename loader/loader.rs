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
//    FIXME: handle_protocol() ( called by initialize_lib() ) is always hang up now
//    uefi::initialize_lib(&hdl, &sys);
    uefi::set_system_table(&sys);

    let bs = uefi::get_system_table().boot_services();
    let rs = uefi::get_system_table().runtime_services();

    uefi::get_system_table().console().write("Hello, World!\n\r");

    let (memory_map, memory_map_size, map_key, descriptor_size, descriptor_version) = uefi::lib_memory_map();

    bs.exit_boot_services(&hdl, &map_key);
    rs.set_virtual_address_map(&memory_map_size, &descriptor_size, &descriptor_version, memory_map);

    rs.reset_system(uefi::ResetType::Shutdown, uefi::Status::Success);

    loop {
    }
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
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    loop {}
}
