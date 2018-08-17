#![no_std]
#![no_main]

#![feature(slice_patterns)]
#![feature(alloc)]
#![feature(asm)]
#![feature(extern_prelude)]

extern crate uefi;
extern crate uefi_exts;
#[macro_use]
extern crate log;
#[macro_use]
extern crate alloc;

use uefi::prelude::*;
use uefi::table::boot::MemoryDescriptor;
use uefi::proto::console::gop::GraphicsOutput;
use uefi_exts::BootServicesExt;
use core::mem;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn efi_main(handle: uefi::Handle, system_table: &'static SystemTable) -> Status {
    uefi_services::init(system_table);
    let graphic_mode = system_table.stdout().modes().last().unwrap();
    system_table.stdout().set_mode(graphic_mode).expect("Failed");
    gop_init(system_table.boot);
    memmap(system_table.boot);
    loop{}
    return uefi::Status::Success;
}

fn gop_init(bt: &BootServices) {
    if let Some(mut gop_proto) = bt.find_protocol::<GraphicsOutput>() {
        let gop = unsafe { gop_proto.as_mut() };
        let mode = gop.modes()
            .find(|ref mode| {
                let mode_info = mode.info();
                mode_info.resolution() == (1280, 720)
            }).unwrap();
        gop.set_mode(&mode).expect("failed");
    } else {
        warn!("UEFI GOP is not supported");
    }
}

fn memmap(bt: &BootServices) {
    let map_size = bt.memory_map_size();
    let buffer_size = map_size + 8 * mem::size_of::<MemoryDescriptor>();
    let mut buffer = Vec::with_capacity(buffer_size);

    unsafe {
        buffer.set_len(buffer_size);
    }

    let (_key, mut descriptor_iter) = bt.memory_map(&mut buffer)
        .expect("failed");

    assert!(descriptor_iter.len() > 0, "memory map is empty");

    let first_descriptor = descriptor_iter.next().unwrap();
    let physical_address_start = first_descriptor.phys_start;

    assert_eq!(physical_address_start, 0, "memory does not start at address 0x0");
}
