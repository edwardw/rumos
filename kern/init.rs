#[crate_id="kern64"];
#[crate_type="staticlib"];
#[no_std];
#[feature(globs)];

extern mod std;
extern mod extra;
extern mod arch;

use std::cast;
use std::int;
use std::option::{Some, None};
use std::iter::Iterator;
use std::vec::ImmutableVector;
use extra::term;

static SPLASH0: &'static str = "    ____                  ____  _____\n";
static SPLASH1: &'static str = "   / __ \\__  ______ ___  / __ \\/ ___/\n";
static SPLASH2: &'static str = "  / /_/ / / / / __ `__ \\/ / / /\\__ \\\n";
static SPLASH3: &'static str = " / _, _/ /_/ / / / / / / /_/ /___/ /\n";
static SPLASH4: &'static str = "/_/ |_|\\__,_/_/ /_/ /_/\\____//____/\n";

static FORTUNE: &'static str = "\n2014 = 1024 + 512 + 256 + 128 + 64 + 16 + 8 + 4 + 2!\n";

#[no_mangle]
pub extern "C" fn init() {
    use arch::drivers::vga;

    init_bss();

    vga::init();
    vga::puts(SPLASH0, term::color::WHITE);
    vga::puts(SPLASH1, term::color::WHITE);
    vga::puts(SPLASH2, term::color::WHITE);
    vga::puts(SPLASH3, term::color::WHITE);
    vga::puts(SPLASH4, term::color::WHITE);
    vga::puts(FORTUNE, term::color::BRIGHT_GREEN);
    int::to_str_bytes(2014, 16, |buf| for c in buf.iter() { vga::putc(*c as char, term::color::WHITE); });

    loop {}
}

//
// Use linker provided symbols to zero the bss section, so the kernel
// is properly loaded.
//
fn init_bss() {
    use arch::cpu;
    extern {
        static edata: u64;
        static end: u64;
    }

    unsafe {
        // The linker provided symbols are actually pointers.
        // What matters is where they point to, not what.
        let edata_ptr = cast::transmute::<*u64, uint>(&edata);
        let end_ptr = cast::transmute::<*u64, uint>(&end);
        cpu::memset(edata_ptr, 0, end_ptr - edata_ptr);
    }
}
