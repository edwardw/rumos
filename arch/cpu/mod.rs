pub mod io;

#[inline(always)]
pub unsafe fn memmove(dst: uint, src: uint, n: uint) {
    if src < dst && src + n > dst {
        asm!("std; rep movsb"
            :
            : "{Di}" (dst+n-1), "{Si}" (src+n-1), "{ecx}" (n)
            : "memory", "cc"
            : "volatile");
    } else {
        asm!("cld; rep movsb"
            :
            : "{Di}" (dst), "{Si}" (src), "{ecx}" (n)
            : "memory", "cc"
            : "volatile");
    }
}

#[inline(always)]
pub unsafe fn memset(va: uint, c: uint, n: uint) {
    asm!("cld; rep stosb"
        :
        : "{rDi}" (va), "{rax}" (c), "{rcx}" (n)
        : "memory", "cc"
        : "volatile");
}

static IO_PIC1: u16 = 0x20;
static IO_PIC2: u16 = 0xA0;
static IRQ_SLAVE: u8 = 2;
pub static mut IRQ_MASK_8259A: u16 = 0xFFFF & !(1<<IRQ_SLAVE);
static mut PIC_INIT_DONE: bool = false;

pub unsafe fn irq_setmask_8259A(mask: u16) {
    IRQ_MASK_8259A = mask;
    if PIC_INIT_DONE {
        io::outb(IO_PIC1+1, mask as u8);
        io::outb(IO_PIC2+2, (mask>>8) as u8);
    }
}
