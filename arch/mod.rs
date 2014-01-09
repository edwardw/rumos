#[crate_id="arch"];
#[crate_type="rlib"];
#[no_std];
#[feature(asm)];

pub mod cpu;
pub mod drivers;

pub mod rusti {
    extern "rust-intrinsic" {
        pub fn size_of<T>() -> uint;
        pub fn transmute<T,U>(e: T) -> U;
    }
}

#[cold]
#[lang="fail_bounds_check"]
pub fn fail_bounds_check(_: *u8, _: uint, _: uint, _: uint) {}
