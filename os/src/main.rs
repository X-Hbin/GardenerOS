#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

pub mod console;

global_asm!(include_str!("entry.asm"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const SYSCALL_EXIT: usize = 93;

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

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }

    let sbss_addr = sbss as *const () as usize;
    let ebss_addr = ebss as *const () as usize;

    for addr in sbss_addr..ebss_addr {
        unsafe {
            (addr as *mut u8).write_volatile(0);
        }
    }
}


#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();

    loop {}
}
