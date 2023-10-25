#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga;

// Imported modules
use bootloader_api::{config, info};

#[allow(dead_code)] //
const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiBS
    config
};
bootloader_api::entry_point!(kernel_main);

#[allow(unreachable_code)] //
#[allow(unused_variables)] //
fn kernel_main(boot_info: &'static mut info::BootInfo) -> ! {
    // start of initialization of required parameters
    {
        use vga::{FrameBufferWriter, WRITER};

        *WRITER.lock() = match &mut boot_info.framebuffer {
            info::Optional::Some(framebuffer) => {
                let framebuffer_info = framebuffer.info().clone();
                Option::Some(FrameBufferWriter::new(
                    framebuffer.buffer_mut(),
                    framebuffer_info,
                ))
            }
            info::Optional::None => panic!("Failed to get framebuffer from boot info."),
        };
    }
    // end of initialization of required parameters

    println!("Hello World{}", "я русский");
    panic!("some panic");

    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
