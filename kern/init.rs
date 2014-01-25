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

pub mod util;

//
// GNU grub2 multiboot info structure
// http://www.gnu.org/software/grub/manual/multiboot/multiboot.html#Boot-information-format
//
#[packed]
struct MultibootInfo {
    flags               : u32,
    mem_lower           : u32,
    mem_high            : u32,
    boot_device         : u32,
    cmbline             : u32,
    mods_count          : u32,
    mods_addr           : u32,
    syms                : [u8, ..12],
    mmap_length         : u32,
    mmap_addr           : u32,
    drives_length       : u32,
    drives_addr         : u32,
    config_table        : u32,
    boot_loader_name    : u32,
    apm_table           : u32,
    // don't care the rest so omitted
}

static SPLASH0: &'static str = r"    ____                  ____  _____";
static SPLASH1: &'static str = r"   / __ \__  ______ ___  / __ \/ ___/";
static SPLASH2: &'static str = r"  / /_/ / / / / __ `__ \/ / / /\__ \";
static SPLASH3: &'static str = r" / _, _/ /_/ / / / / / / /_/ /___/ /";
static SPLASH4: &'static str = r"/_/ |_|\__,_/_/ /_/ /_/\____//____/";

static FORTUNE: &'static str = "\n2014 = 1024 + 512 + 256 + 128 + 64 + 16 + 8 + 4 + 2!\n";

#[no_mangle]
pub extern "C" fn init(mb_info: *MultibootInfo) {
    use arch::drivers::vga;
    use arch::drivers::keyboard;
    use arch::cpu;

    let kconsole = util::get_kconsole();

    init_bss();

    vga::init();
    kconsole.write_line(SPLASH0);
    kconsole.write_line(SPLASH1);
    kconsole.write_line(SPLASH2);
    kconsole.write_line(SPLASH3);
    kconsole.write_line(SPLASH4);
    vga::puts(FORTUNE, term::color::BRIGHT_GREEN);
    kconsole.write_int(2014);
    for i in [1024, 512, 256, 128, 64, 16, 8, 4, 2].iter() {
        kconsole.write_str(" 0b");
        int::to_str_bytes(*i, 8, |buf| kconsole.write(buf));
    }

    keyboard::init();

    let (basemem, extmem, extmem_16mplus) = cpu::detect_memory();
    kconsole.write_str("\n\nBase memory (K): ");
    kconsole.write_uint(basemem / 1024);
    kconsole.write_str("\nExtended memory (K): ");
    kconsole.write_uint(extmem / 1024);
    kconsole.write_str("\nExtended memory >16M (K): ");
    kconsole.write_uint(extmem_16mplus / 1024);

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
