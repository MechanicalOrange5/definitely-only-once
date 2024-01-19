## definitely-only-once

### DO NOT USE THIS FOR ANYTHING. IT IS BAD, UNSAFE AND I DON'T KNOW WHAT I AM DOING

definitely-only-once is a crate for all your needs of only running code once. Sure it's possible with safe code, and it will be better and faster, but here is a worse way in every single aspect. It's wildly unsafe, not portable, will likely only run on x86_64 linux when the stars align properly. There is no guarantee that that the code inside this macro will execute correctly, or that the compiler won't hand in it's resignation immediately. 

How it works is by using some self modifying assembly code. In essence in pseudo assembly it goes something like this

```asm
start:
jmp real_start
jmp end
real_start:
mov byte ptr [rip + start ], 90h //this line and the one below turn jmp real_start into nop (no-operation)
mov byte ptr [rip + start +1], 90h
<your code here>
end:
```
It will run and jump to the real_start: label, where subsequently it will overide that instruction with nop instructions (no operation), and then execute your code. The next time it runs, it will run the nop's, and immediately jump to the end: label, never ever running your code again, no matter what (or unless you do equally funny shenannigans)

There is a bit more involved in making the code executing writeable, because for some reason linux doesn't like it when you overide executing code.

Do not use this for anything ever. This is meant as a joke and a funny example of self modifying code. And I am really bad at assembly. Just putting it out there.

# Examples

```rust
fn main() {
    for _ in 0..1000 {
        // always use unique idents for every invocation of the macro. 
        execute_once!("print", {
            println!("Hello from loop!");
        });
    }
}
```
prints
```
Hello from loop!
```
## once


It work for functions too
```rust
// inlining causes issues with labels
#[inline(never)]
pub fn fn_example() {
    execute_once!("fn", {
        println!("Hello, from a function!");
    });
}


fn main() {
    fn_example();
    fn_example();
}
```
prints
```
Hello, from a function!
```
## once


Sometimes the compiler gets in the way. On --release this won't compile. This is due to the fact that it is doing loop unrolling (probably), and because you cannot have duplicate labels in asm! it turns into a compile error. --debug is fine
```rust
static mut COUNTER: usize = 0;
fn main() {
    for _ in 0..10 {
        execute_once!("optimize", {
            COUNTER += 1;
        });
        
    }
    println!("counter is {}", unsafe {COUNTER})
}
```
but this will compile fine on release. This code also executes a lot faster on debug for some reason. Reverse optimizations :)
```rust
static mut COUNTER: usize = 0;
fn main() {
    for _ in 0..100 {
        execute_once!("optimize", {
            COUNTER += 1;
        });
        
    }
    println!("counter is {}", unsafe {COUNTER})
}
```
and prints
```
counter is 1
```
## once


Have fun and remember to not use this for anything.

## License

This project is licensed under either of the following licenses, at your option:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT])