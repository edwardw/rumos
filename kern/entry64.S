#include <inc/mmu.h>

.global start64
    .code64
start64:
    # Jump to the final virtual address of the kernel.
    # After that we're in the higher half of the address space.
    movq    $relocated, %rax
    jmp     *%rax
relocated:
    # Set up the stack and call into rust
    movq    $bootstacktop, %rsp
    xorq    %rax, %rax
    movq    %rax, %rbp
    movq    %rax, %fs:0x70
    call    init

    # Spin if init returns (it shouldn't).
spin:
    jmp     spin

.data

    .p2align PG4K_SHIFT
bootstack:
    .space      PG4K_SIZE
bootstacktop:
