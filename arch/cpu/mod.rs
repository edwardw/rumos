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
        // XXX this is a hack
        // don't know why need (n & 0xFFFF) for the function to work properly
        : "{rDi}" (va), "{rax}" (c), "{rcx}" (n & 0xFFFF)
        : "memory", "cc"
        : "volatile");
}
