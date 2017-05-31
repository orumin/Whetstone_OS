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

    let bs = uefi::get_system_table().boot_services();
    let rs = uefi::get_system_table().runtime_services();

    uefi::get_system_table().console().write("Hello, World!\n\r");

    let (memory_map, memory_map_size, map_key, descriptor_size, descriptor_version) = uefi::lib_memory_map();

    let gop = match uefi::graphics::GraphicsOutputProtocol::new() {
        Ok(r) => r,
        Err(_) => panic!(),
    };

    for _ in 0..gop.get_max_mode() {
        uefi::get_system_table().console().write("Ya!\n\r");
    }

    let mut mode: u32 = 0;
    for i in 0..gop.get_max_mode() {
        let info = match gop.query_mode(i) {
            Ok(r) => r,
            Err(r) => return r,
        };

        if info.pixel_format != uefi::graphics::PixelFormat::RedGreenBlue
            && info.pixel_format != uefi::graphics::PixelFormat::BlueGreenRed { continue; }
        if info.horizontal_resolution > 1920 && info.vertical_resolution > 1080 { continue; }
        if info.horizontal_resolution == 1920 && info.vertical_resolution == 1080 { mode = i; break; }
        mode = i;
    };

    gop.set_mode(mode);
    gop.draw(&[uefi::graphics::Pixel::new(255, 0, 0) ; 300], 0, 300, 0, 0);

    //rs.reset_system(uefi::ResetType::Shutdown, uefi::Status::Success);

    bs.exit_boot_services(&hdl, &map_key);
    rs.set_virtual_address_map(&memory_map_size, &descriptor_size, &descriptor_version, memory_map);

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
