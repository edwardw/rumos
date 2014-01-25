#[crate_id="kern64"];
#[crate_type="staticlib"];
#[no_std];
#[feature(globs)];

extern mod std;
extern mod extra;
extern mod arch;

use std::cast;
use std::{int, uint};
use std::option::{Some, None};
use std::iter::Iterator;
use std::vec::ImmutableVector;
use std::ptr::RawPtr;
use extra::term;
use arch::drivers::vga;

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
    syms                : [u8, ..16],
    mmap_length         : u32,
    mmap_addr           : u32,
    drives_length       : u32,
    drives_addr         : u32,
    config_table        : u32,
    boot_loader_name    : u32,
    apm_table           : u32,
    // don't care the rest so omitted
}

#[packed]
struct GrubMemMap {
    mmap_size   : u32,
    mmap_addr   : u64,
    mmap_length : u64,
    mmap_type   : u32,
}

static SPLASH0: &'static str = r"    ____                  ____  _____";
static SPLASH1: &'static str = r"   / __ \__  ______ ___  / __ \/ ___/";
static SPLASH2: &'static str = r"  / /_/ / / / / __ `__ \/ / / /\__ \";
static SPLASH3: &'static str = r" / _, _/ /_/ / / / / / / /_/ /___/ /";
static SPLASH4: &'static str = r"/_/ |_|\__,_/_/ /_/ /_/\____//____/";

static FORTUNE: &'static str = "\n2014 = 1024 + 512 + 256 + 128 + 64 + 16 + 8 + 4 + 2!\n";

#[no_mangle]
pub extern "C" fn init(mb_info: *MultibootInfo) {
    use arch::drivers::keyboard;

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
    kconsole.write_line("");

    init_mmap(mb_info);

    keyboard::init();

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

//
// Interpret the grub mmap and initialize vmem.
//
fn init_mmap(mb_info: *MultibootInfo) {
    unsafe {
        if (*mb_info).flags & (1<<6) != 0 {
            // the mmap info exists
            let kconsole = util::get_kconsole();
            kconsole.write_line("Memory maps:");

            let mut mmap_len = (*mb_info).mmap_length;
            let mut mmap = (*mb_info).mmap_addr as *GrubMemMap;
            while mmap_len as i32 > 0 {
                let mem_start = (*mmap).mmap_addr;
                let mem_end = (*mmap).mmap_addr + (*mmap).mmap_length;
                let mem_type = (*mmap).mmap_type;

                mmap_len -= (*mmap).mmap_size;
                kconsole.write_str("    ");
                uint::to_str_bytes(mem_start as uint, 16, |buf| kconsole.write(buf));
                kconsole.write_str(" -> ");
                uint::to_str_bytes(mem_end as uint, 16, |buf| kconsole.write(buf));
                kconsole.write_str(", type ");
                kconsole.write_uint(mem_type as uint);
                kconsole.write_line("");
                mmap = mmap.offset(1);
            }
        }
    }
}
