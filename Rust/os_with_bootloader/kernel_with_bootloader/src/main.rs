#![no_std]
#![no_main]

mod writer;
use writer::FrameBufferWriter;


use bootloader_api::config::Mapping;
use x86_64::instructions::hlt;


// mod interrupts;
// use interrupts::{init_idt, PICS};

// Use the entry_point macro to register the entry point function:
//bootloader_api::entry_point!(kernel_main)

pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; //100KiB
    config
};
bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();
    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();
    let mut frame_buffer_writer = FrameBufferWriter::new(buffer, frame_buffer_info);

    // use core::fmt::Write; // Required for `writeln!`


    // init_idt();
    // unsafe {PICS.lock().intitialize()};
    // x86_64::instructions::interrupts::enable();

    // Write multiple lines to test scrolling
    for i in 0..20 {
        frame_buffer_writer.print("\nHello world \nThis is a test \n\\cOrange Text \\rIndented Text");

}
    
    // Halt the CPU in an infinite loop
    loop {
        x86_64::instructions::hlt();
    }
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
loop{
hlt();
}
}