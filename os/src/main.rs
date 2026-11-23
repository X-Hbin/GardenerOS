#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

pub mod console;

// 嵌入汇编入口代码
global_asm!(include_str!("entry.asm"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const SYSCALL_EXIT: usize = 93;

// 暂时保留原有系统调用封装
pub(crate) fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;

    unsafe {
        asm!(
            "ecall",
            in("x10") args[0],
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
            lateout("x10") ret
        );
    }

    ret
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

// Rust 内核入口
#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    loop {}
}


