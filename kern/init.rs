#[crate_id="kern64"];
#[crate_type="staticlib"];
#[no_std];
#[feature(globs)];

extern mod core;
extern mod std;
extern mod arch;

use std::prelude::*;

static SPLASH0: &'static str = "    ____                  ____  _____\n";
static SPLASH1: &'static str = "   / __ \\__  ______ ___  / __ \\/ ___/\n";
static SPLASH2: &'static str = "  / /_/ / / / / __ `__ \\/ / / /\\__ \\\n";
static SPLASH3: &'static str = " / _, _/ /_/ / / / / / / /_/ /___/ /\n";
static SPLASH4: &'static str = "/_/ |_|\\__,_/_/ /_/ /_/\\____//____/\n";

static FORTUNE: &'static str = "\n2014 = 1024 + 512 + 256 + 128 + 64 + 16 + 8 + 4 + 2!\n";

#[no_mangle]
pub extern "C" fn init() {
    use arch::drivers::vga;
    use arch::cpu;

    // Use linker provided symbols to zero the bss section, so the kernel is
    // properly loaded.
    extern {
        static edata: u64;
        static end: u64;
    }
    unsafe {
        cpu::memset(edata as uint, 0, (end - edata) as uint);
    }

    vga::init();
    vga::puts(SPLASH0, term::color::WHITE);
    vga::puts(SPLASH1, term::color::WHITE);
    vga::puts(SPLASH2, term::color::WHITE);
    vga::puts(SPLASH3, term::color::WHITE);
    vga::puts(SPLASH4, term::color::WHITE);
    vga::puts(FORTUNE, term::color::BRIGHT_GREEN);

    loop {}
}
