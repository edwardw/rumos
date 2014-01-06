#[crate_type="staticlib"];
#[feature(asm)];
#[no_std];

#[packed]
struct Elf64 {
    e_magic     : u32,
    e_elf       : [u8, ..12],
    e_type      : u16,
    e_machine   : u16,
    e_version   : u32,
    e_entry     : u64,
    e_phoff     : u64,
    e_shoff     : u64,
    e_flags     : u32,
    e_ehsize    : u16,
    e_phentsize : u16,
    e_phnum     : u16,
    e_shentsize : u16,
    e_shnum     : u16,
    e_shstrndx  : u16,
}

#[packed]
struct Proghdr64 {
    p_type      : u32,
    p_flags     : u32,
    p_offset    : u64,
    p_va        : u64,
    p_pa        : u64,
    p_filesz    : u64,
    p_memsz     : u64,
    p_align     : u64,
}

mod rusti {
    extern "rust-intrinsic" {
        pub fn size_of<T>() -> uint;
        pub fn transmute<T,U>(e: T) -> U;
    }
}

#[no_mangle]
pub extern "C" fn bootmain(elfhdr: *mut Elf64) -> u32 {
    unsafe {
        let mut ph: *Proghdr64;

        // Read off one page of the kernel.
        readseg(rusti::transmute(elfhdr), 4096, 0);

        ph = rusti::transmute(
            rusti::transmute::<*mut Elf64, u32>(elfhdr) +
            (*elfhdr).e_phoff as u32);
        while (*elfhdr).e_phnum > 0 {
            readseg((*ph).p_pa as u32, (*ph).p_memsz as u32, (*ph).p_offset as u32);
            ph = rusti::transmute(
                rusti::transmute::<*Proghdr64, u32>(ph) +
                rusti::size_of::<Proghdr64>() as u32);
            (*elfhdr).e_phnum -= 1;
        }

        (*elfhdr).e_entry as u32
    }
}

//
// Read 'count' bytes at 'offset' from kernel into physical address 'pa'.
// Might copy more than asked.
//
unsafe fn readseg(pa: u32, count: u32, offset: u32) {
    // Round down to the sector boundary
    let addr = pa & !(512 - 1);
    // Translate bytes to sectors, and kernel starts at sector 1
    let start_sect = (offset >> 9) + 1;
    // Round up to the sector boundary
    let nsect = (count >> 9) + if count&512==0 { 0 } else { 1 };
    let mut i = 0;
    while i < nsect {
        readsect(addr + 512 * i, start_sect + i);
        i += 1;
    }
}

//
// Read one sector at 'offset' of the first HDD into physical address 'dst'.
//
unsafe fn readsect(dst: u32, offset: u32) {
    waitdisk();

    outb(0x1F2, 1);
    outb(0x1F3, offset as u8);
    outb(0x1F4, (offset >> 8) as u8);
    outb(0x1F5, (offset >> 16) as u8);
    outb(0x1F6, ((offset >> 24) | 0xE0) as u8);
    outb(0x1F7, 0x20);

    waitdisk();

    // Read one sector: 512 / 4 = 128
    insl(0x1F0, dst, 128);
}

#[inline(always)]
unsafe fn waitdisk() {
    while (inb(0x1F7) & 0xC0) != 0x40 {}
}

#[inline(always)]
unsafe fn inb(port: u16) -> u8 {
    let data: u8;
    asm!("inb $1,$0" : "={ax}" (data) : "{dx}" (port) :: "volatile");
    data
}

#[inline(always)]
unsafe fn outb(port: u16, data: u8) {
    asm!("outb $0,$1" :: "{ax}" (data), "{dx}" (port) :: "volatile");
}

#[inline(always)]
unsafe fn insl(port: u16, mut addr: u32, mut cnt: int) {
    asm!("cld;
            repne;
            insl"
        : "={Di}" (addr), "={cx}" (cnt)
        : "{dx}" (port), "0" (addr), "1" (cnt)
        : "memory", "cc"
        : "volatile");
}

/*
 * Unfortunately, INT 13H only work in real mode.
//
// Disk Address Packet
//  https://en.wikipedia.org/wiki/Int_13h
//
#[packed]
struct DAP {
    sz          : u16,
    nsect       : u16,
    offset      : u16,
    seg         : u16,
    start_sect  : u64,
}

#[no_mangle]
pub extern "C" fn bootmain(elfhdr: *Elf64) -> u32 {
    unsafe {
        let mut ph: *Proghdr64;

        // Read the first page of the kernel.
        readseg(rusti::transmute(elfhdr), 4096, 0);

        let mut i = 0;
        ph = rusti::transmute(rusti::transmute::<*Elf64, u32>(elfhdr)
            + (*elfhdr).e_phoff as u32);
        while i < (*elfhdr).e_phnum {
            readseg((*ph).p_pa as u32, (*ph).p_memsz as u32,
                (*ph).p_offset as u32);
            ph = rusti::transmute(rusti::transmute::<*Proghdr64, u32>(ph)
                + rusti::size_of::<Proghdr64>() as u32);
            i += 1;
        }

        (*elfhdr).e_entry as u32
    }
}

//
// Read 'count' bytes at 'offset' from kernel into physical address 'pa'.
// Might copy more than asked.
//
unsafe fn readseg(pa: u32, count: u32, offset: u32) {
    // Translate bytes to sectors, and kernel starts at sector 1
    let start_sect = (offset >> 9) + 1;
    // Round up to the sector boundary
    let nsect = (count >> 9) + if count&512==0 { 0 } else { 1 };
    // Round down to the sector boundary
    let addr = pa & !(512 - 1);
    
    // 0x80 = the first HDD
    readsect(0x80, start_sect, nsect as u16, addr);
}

//
// Read 'nsect' sectors from HDD 'driver' starting from the 'start_sect'
// sector, and copy the data into physical address 'pa'.
//
unsafe fn readsect(driver: u8, start_sect: u32, nsect: u16, pa: u32) {
    let dap = DAP {sz: 16, nsect: nsect, offset: pa as u16, seg: 0, start_sect: start_sect as u64};
    asm!("int $0;"
        :
        : "i" (0x13), "{ah}" (0x42), "{dl}" (driver),
            "{Si}" (rusti::transmute::<*DAP, u32>(&dap))
        : "memory", "cc"
        : "volatile");
}
*/
