/*
 * We need to disable the standard library because of its interoperation with the underlying
 * operating system: (we are the operating system now)
 */
#![no_std]
#![no_main]
mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

/*
* The rust runtime starts in a C runtime library called `crt0` (see
* https://github.com/rust-lang/rust/blob/bb4d1491466d8239a7a5fd68bd605e3276e97afb/src/libstd/rt.rs#L32-L73)
* We need to override `crt0` directly by defining _start
*
*   Aside on `crt0`: `crt0` is a object file written in assembly that is implicitly included
*   by the linker at runtime. `crt0` contains the most basic parts of a runtime library
*
*   A basic `crt0.s` might be:
*
*   ```asm
*   .text
*
*   .globl _start
*
*   _start: # _start is the entry point known to the linker
*       xor %ebp, %ebp            # effectively RBP := 0, mark the end of stack frames
*       mov (%rsp), %edi          # get argc from the stack (implicitly zero-extended to 64-bit)
*       lea 8(%rsp), %rsi         # take the address of argv from the stack
*       lea 16(%rsp,%rdi,8), %rdx # take the address of envp from the stack
*       xor %eax, %eax            # per ABI and compatibility with icc
*       call main                 # %edi, %rsi, %rdx are the three args
*                                 # (of which first two are C standard) to main
*       mov %eax, %edi    # transfer the return of main to the first argument of _exit
*       xor %eax, %eax    # per ABI and compatibility with icc
*       call _exit        # terminate the program
*   ```
*/
#[no_mangle]
// We use extern "C" to ensure that the rust compiler uses C calling conventions
pub extern "C" fn _start() -> ! {
    loop {}
}
