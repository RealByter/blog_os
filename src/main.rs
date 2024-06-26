#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use blog_os::{
    memory, println, scheduling::SCHEDULER, task::{executor::Executor, keyboard, Task}
};
use bootloader::{entry_point, BootInfo};
use x86_64::{structures::paging::{page, Mapper, Page, PageTableFlags as Flags, PhysFrame, Size4KiB, FrameAllocator}, PhysAddr};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::{allocator, memory::BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    
    blog_os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // let heap_value = Box::new(41);
    // println!("heap_value at {:p}", heap_value);

    // let mut vec = Vec::new();
    // for i in 0..500 {
    //     vec.push(i);
    // }
    // println!("vec at {:p}", vec.as_slice());

    // let reference_counted = Rc::new(vec![1, 2, 3]);
    // let cloned_reference = reference_counted.clone();
    // println!(
    //     "current reference count is {}",
    //     Rc::strong_count(&cloned_reference)
    // );
    // core::mem::drop(reference_counted);
    // println!(
    //     "reference count is {} now",
    //     Rc::strong_count(&cloned_reference)
    // );
    #[cfg(test)]
    test_main();

    // let frame: PhysFrame<Size4KiB> = frame_allocator.allocate_frame().unwrap();
    // let page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(0x_8444_4444_0000));
    // let flags = Flags::PRESENT | Flags::WRITABLE;
    // let map_to_result = unsafe { mapper.map_to(page, frame, flags, &mut frame_allocator)};
    // map_to_result.expect("map_to failed").flush();

    SCHEDULER.lock().spawn(|| {
        loop {
        }
    });

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
