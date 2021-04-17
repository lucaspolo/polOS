#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pol_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::{panic::PanicInfo};
use pol_os::{memory::BootInfoFrameAllocator, println, task::{Task, executor::Executor, keyboard}};
use bootloader::{BootInfo, entry_point};

extern crate alloc;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use pol_os::allocator;
    use pol_os::memory;
    use x86_64::{VirtAddr};

    println!("Hello World{}", "!");
    pol_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    pol_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pol_os::test_panic_handler(info)
}

async fn async_numer() -> u32 {
    42
}

async fn example_task() {
    let number = async_numer().await;
    println!("async number: {}", number);
}