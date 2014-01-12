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
