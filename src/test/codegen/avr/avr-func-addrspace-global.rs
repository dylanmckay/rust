// compile-flags: -O --target=avr-unknown-unknown --crate-type=rlib

// This test validates that function pointers can be stored in global variables
// and called upon. It ensures that Rust emits function pointers in the correct
// address space to LLVM so that an assertion error relating to casting is
// not triggered.

#![feature(no_core, lang_items, unboxed_closures, arbitrary_self_types)]
#![crate_type = "lib"]
#![no_core]

#[lang = "sized"]
pub trait Sized { }
#[lang = "copy"]
pub trait Copy { }
#[lang = "receiver"]
pub trait Receiver { }

impl Copy for usize {}

#[lang = "drop_in_place"]
pub unsafe fn drop_in_place<T: ?Sized>(_: *mut T) {}


#[lang = "fn_once"]
pub trait FnOnce<Args> {
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}

#[lang = "fn_mut"]
pub trait FnMut<Args> : FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

#[allow(dead_code)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub static mut FOO: fn(&usize, &mut u32) -> Result<(), ()> = calc_foo;

fn calc_foo(ptr: &usize, _: &mut u32) -> Result<(), ()> {
    let raw_ptr = ptr as *const usize;
    let _v: usize = unsafe { *raw_ptr };
    loop {}
}

// CHECK: define void @test(){{.+}}addrspace(1)
#[no_mangle]
pub extern "C" fn test() {
    let mut buf = 7;
    // CHECK: load {{.*}}addrspace(1){{.+}}FOO
    unsafe {
        FOO(&1, &mut buf);
    }
}
