#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod lang_items;
pub mod syscall;
pub mod signal;

use linked_list_allocator::LockedHeap;

#[global_allocator]
/// heap allocator instance
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap(size: usize) {
    let start = sys_mmap(size, 0x3);

    unsafe {
        HEAP_ALLOCATOR.lock().init(start as usize as *mut u8, size);
    }
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    init_heap(4096 * 32);
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

use syscall::*;

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

pub fn r#yield() {
    sys_yield()
}

pub fn fork() -> isize {
    sys_fork()
    
}

pub fn exec(addr: usize) -> isize {
    sys_exec(addr)
}

pub fn wait(pid: usize) -> isize {
    sys_wait(pid)
}

pub fn create_pipe(fd: &mut [usize]) -> isize {
    sys_create_pipe(fd)
}