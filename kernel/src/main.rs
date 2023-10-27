#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
            // #![feature(custom_test_frameworks)]
            // #![test_runner(crate::test_runner)]
            // #![reexport_test_harness_main = "test_main"]

pub struct Flag {
    short: char,
    name: &'static str,
    /* ... */
}
unsafe impl Send for Flag {}
unsafe impl Sync for Flag {}

impl Flag {
    pub const fn new(short: char, name: &'static str) -> Self {
        Flag { short, name }
    }
}

inventory::collect!(Flag);

inventory::submit! {
    Flag::new('v', "verbose")
}

mod vga_buffer;

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
        use vga_buffer::{FrameBufferWriter, WRITER};

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

    let mut count = 0;
    for flag in inventory::iter::<Flag> {
        count += 1;
    }
    println!("{}", count);

    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// inventory::collect!(TestFlag);
// inventory::submit! {{
//     let func: &'static dyn Fn() = &|| {println!("Hello from the closure!")};
//     TestFlag {
//         name: "amogus"
//     }
// }}
// #[derive(Debug)]
// struct TestFlag {
//     // func: &'static dyn Fn(),
//     name: &'static str,
// }
// impl TestFlag {
//     //     fn new(func: &'static dyn Fn()) -> Self {
//     //         TestFlag { func: func }
//     //     }
//     //     fn get_func<'a>(self) -> &'a dyn Fn() {
//     //         self.func
//     //     }
//     pub fn name(&self) -> &str {
//         self.name
//     }
// }
// unsafe impl Send for TestFlag {}
// unsafe impl Sync for TestFlag {}
// // macro_rules! test {
//     ($name:ident, $body:block) => {
//         fn $name() {
//             $body
//         }
//         // inventory::submit! {
//         //     TestFlag(&|| $body)
//         // }
//         inventory::submit! {TestFlag($name)}
//     };
// }
// test!(test_addition, {
//     print!("trivial assertion... ");
//     // assert_eq!(1, 1);
//     println!("[ok]");
// });

// #[cfg(test)]
// fn test_runner(tests: &[&dyn Fn()]) {
//     println!("Running {} tests", tests.len());
//     for test in tests {
//         test();
//     }
// }

// fn trivial_assertion() {
//     print!("trivial assertion... ");
//     assert_eq!(1, 1);
//     println!("[ok]");
// }
