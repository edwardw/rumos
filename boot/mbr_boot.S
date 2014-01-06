# 2014-01-02 3:53

#include <inc/mmu.h>

.global start
    .code16
start:
    cli

    # Set up the data segment registers
    xorw    %ax, %ax
    movw    %ax, %es
    movw    %ax, %ds
    movw    %ax, %ss

    # Switch to the protected mode.
    lgdt    gdt32_desc
    movl    %cr0, %eax
    orl     $CR0_PE_ON, %eax
    movl    %eax, %cr0

    ljmp    $PROT_MODE_CSEG, $start32

    .code32
start32:
    movw    $PROT_MODE_DSEG, %ax
    movw    %ax, %ds
    movw    %ax, %es
    movw    %ax, %fs
    movw    %ax, %gs
    movw    %ax, %ss

    # Set up the stack and call into Rust
    movl    $start, %esp
    push    $0x10000   # argv0 for bootmain, the scratch space
    # It seems (%gs:0x30) is where rust stores the value of stack bottom.
    # Set it to zero to convince rust there's enough stack space.
    movl    $0, %gs:0x30
    call    bootmain
    # Bootmain is expected to return the kernel entry point in eax
    call    *%eax

    # The kernel should never return. It if does, spin.
spin:
    jmp spin

# LLVM generates prologue code for every function which may call into
# __morestack. It does has 'naked' attribute for functions to disable
# prologue code. Until rustc makes use of that attribute, this stub
# is needed to keep rustc happy.
# Check out
#       rust/src/rt/arch/i386/morestack.S
#       rust/src/rt/arch/x86_64/morestack.S
# for more details. The rust intrinsic morestack_addr is also relevant.
.global __morestack
__morestack:
    ret

# Protected mode bootstrap GDT
# 4 bytes alignment
.p2align 2
gdt32:
    SEG32_NULL
    SEG32(STA_X|STA_R, 0x0, 0xFFFFFFFF) # code seg
    SEG32(STA_W, 0x0, 0xFFFFFFFF)   # data seg
gdt32_end:

gdt32_desc:
    .word gdt32_end - gdt32 - 1
    .long gdt32
