#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga;

#[allow(unused_imports)] //
use bootloader_api::{config, info};
#[allow(unused_imports)] //
use core::fmt::Write;
#[allow(unused_imports)] //
use vga::FrameBufferWriter;

#[allow(dead_code)] //
const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};
bootloader_api::entry_point!(kernel_main);

#[allow(unused_variables)] //
fn kernel_main(boot_info: &'static mut info::BootInfo) -> ! {
    // vga object
    let mut framebuffer_writer = match &mut boot_info.framebuffer {
        info::Optional::Some(frame_buffer) => {
            let framebuffer_info = frame_buffer.info().clone();
            FrameBufferWriter::new(frame_buffer.buffer_mut(), framebuffer_info)
        }
        info::Optional::None => panic!("Failed to get framebuffer from boot info."),
    };

    // write example on vga
    framebuffer_writer
        .write_str("-Staralis, Klim Sanich \n-Da \n-Pyatikratno pervarenniy KAL \n-DA")
        .expect("FrameBufferWriter 'write_str()' error");

    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
