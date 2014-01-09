#[crate_id="kern"];
#[crate_type="staticlib"];
#[no_std];

extern mod arch;

static SPLASH: &'static str = "2014 = 1024 + 512 + 256 + 128 + 64 + 16 + 8 + 4 + 2!";

#[no_mangle]
pub extern "C" fn init() {
    use arch::drivers::vga;
    use arch::rusti;

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
