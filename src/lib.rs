#![allow(unused_imports, unused_macros)]

use std::arch::asm;
use std::hint::black_box;

#[macro_export]
macro_rules! execute_once {
    ($name:literal, $body:tt) => {
        #[allow(named_asm_labels)]
        unsafe {
            asm!(
                concat!($name, "_start:"),
                concat!("jmp ", $name, "_real_start"),
                concat!("jmp ", $name, "_end"),
                concat!($name, "_real_start:"),
                // mprotect syscall start
                "push 0xA",
                "pop rax",
                "push 0x7",
                "pop rdx",
                concat!("call ", $name, "_x"),
                concat!($name, "_x:pop rdi"),
                "and rdi, 0xfffffffffffff000",
                "mov rsi, 0x1000",
                "syscall",
                // mprotect syscall end
                concat!("mov byte ptr [rip + ", $name, "_start ], 90h"),
                concat!("mov byte ptr [rip + ", $name, "_start + 1], 90h"),
                // mprotect syscall start
                "push 0xA",
                "pop rax",
                "push 0x5",
                "pop rdx",
                concat!("call ", $name, "_y"),
                concat!($name, "_y:pop rdi"),
                "and rdi, 0xfffffffffffff000",
                "mov rsi, 0x1000",
                "syscall",
                // mprotect syscall end
            );

            black_box($body);
            asm!(concat!($name, "_end:"));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[inline(never)]
    pub fn fn_example() {
        execute_once!("fn", {
            println!("Hello, from a function!");
        });
    }

    #[test]
    fn print() {
        for _ in 0..1000 {
            execute_once!("print", {
                println!("Hello from loop!");
            });
        }
    }

    #[test]
    fn fn_test() {
        fn_example();
        fn_example();
    }

    static mut COUNTER: usize = 0;
    #[test]
    fn compiler_optimizations() {
        for _ in 0..100 {
            execute_once!("optimize", {
                COUNTER += 1;
            });
        }
        println!("counter is {}", unsafe { COUNTER })
    }
}
