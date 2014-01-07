#[crate_type="staticlib"];
#[no_std];
#[feature(asm)];

#[path="../arch"]
mod arch {
    pub mod cpu;
    pub mod drivers;
}

mod rusti {
    extern "rust-intrinsic" {
        pub fn size_of<T>() -> uint;
        pub fn transmute<T,U>(e: T) -> U;
    }
}

static SPLASH: &'static str = "2014 = 1024 + 512 + 256 + 128 + 64 + 16 + 8 + 4 + 2!";

#[cold]
#[lang="fail_bounds_check"]
fn fail_bounds_check(_: *u8, _: uint, _: uint, _: uint) {}

#[no_mangle]
pub extern "C" fn init() {
    use arch::drivers::vga;

    unsafe {
        vga::init();
        let mut i = 0;
        let msg = rusti::transmute::<&'static str, &'static [u8]>(SPLASH);
        while i < 52 {
            vga::putc(msg[i] as char, vga::LightGreen);
            i += 1;
        }
    }

    loop {}
}
