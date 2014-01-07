#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let data: u8;
    asm!("inb $1,$0" : "={ax}" (data) : "{dx}" (port) :: "volatile");
    data
}

#[inline(always)]
pub unsafe fn outb(port: u16, data: u8) {
    asm!("outb $0,$1" :: "{ax}" (data), "{dx}" (port) :: "volatile");
}
