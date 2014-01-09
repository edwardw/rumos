#[crate_id="kern"];
#[crate_type="staticlib"];
#[no_std];

extern mod core;
extern mod arch;

static SPLASH: &'static str = "2014 = 1024 + 512 + 256 + 128 + 64 + 16 + 8 + 4 + 2!";

#[no_mangle]
pub extern "C" fn init() {
    use core::str;
    use core::container::Container;
    use arch::drivers::vga;

    vga::init();
    let mut i = 0;
    let msg = str::as_bytes(SPLASH);
    while i < (&SPLASH).len() {
        vga::putc(msg[i] as char, vga::LightGreen);
        i += 1;
    }

    loop {}
}
