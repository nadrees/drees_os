#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

/// VGA color Cyan
const CYAN: u8 = 0xb;

fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // writing for VGA. We write the character followed by
            // a color byte, so each character actually takes up 2
            // bytes of space.

            // write the character byte to memory.
            *vga_buffer.offset(i as isize * 2) = byte;
            // write the byte color
            *vga_buffer.offset(i as isize * 2 + 1) = CYAN;
        }
    }

    loop {}
}

bootloader_api::entry_point!(kernel_main);
